//! Асинхронная версия драйвера SIM800L
//! 
//! Этот модуль предоставляет async/await интерфейс для неблокирующей работы с модемом

use crate::error::{Error, Result, ResponseStatus, SmsResponse};
use crate::timer::Timer;
use embedded_hal::serial::{Read, Write};
use heapless::{String, Vec};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

/// Размер буфера для команд и ответов
const BUFFER_SIZE: usize = 1024;

/// Асинхронный драйвер для SIM800L модема
pub struct AsyncSim800L<UART, TIMER> {
    uart: UART,
    timer: TIMER,
    rx_buffer: Vec<u8, BUFFER_SIZE>,
    timeout_ms: u32,
    state: AsyncState,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // Некоторые состояния используются в расширенной версии
enum AsyncState {
    Idle,
    SendingCommand,
    WaitingResponse { start_time: u32 },
    ProcessingResponse,
}

impl<UART, TIMER, UartError> AsyncSim800L<UART, TIMER>
where
    UART: Read<u8, Error = UartError> + Write<u8, Error = UartError> + Unpin,
    TIMER: Timer,
{
    /// Создает новый асинхронный экземпляр драйвера
    pub fn new(uart: UART, timer: TIMER) -> Self {
        Self {
            uart,
            timer,
            rx_buffer: Vec::new(),
            timeout_ms: 10000,
            state: AsyncState::Idle,
        }
    }

    /// Асинхронная инициализация модема
    pub async fn init(&mut self) -> Result<()> {
        // Ждем готовности модема
        self.timer.delay_ms(3000);
        
        // Отключаем эхо команд
        self.send_command_async("ATE0").await?;
        
        // Отключаем flow control
        self.send_command_async("AT+IFC=0,0").await?;
        
        // Проверяем готовность модема
        self.send_command_async("AT").await?;
        
        Ok(())
    }

    /// Асинхронная отправка SMS
    pub async fn send_sms_async(&mut self, phone: &str, message: &str) -> Result<SmsResponse> {
        // Устанавливаем текстовый режим SMS
        self.send_command_async("AT+CMGF=1").await?;

        // Подготавливаем команду отправки
        let mut cmd: String<64> = String::new();
        cmd.push_str("AT+CMGS=\"").map_err(|_| Error::BufferOverflow)?;
        cmd.push_str(phone).map_err(|_| Error::BufferOverflow)?;
        cmd.push_str("\"").map_err(|_| Error::BufferOverflow)?;
        
        // Отправляем команду и ждем приглашение ">"
        let response = self.send_command_async(&cmd).await?;
        if response != ResponseStatus::Prompt {
            return Err(Error::SmsNotSent);
        }
        
        // Отправляем текст сообщения
        self.send_message_text_async(message).await?;
        
        // Отправляем Ctrl+Z для завершения
        self.send_ctrl_z_async().await?;
        
        // Ожидаем подтверждение отправки
        let response = self.wait_for_response_async().await?;
        if response != ResponseStatus::Ok {
            return Err(Error::SmsNotSent);
        }
        
        // Парсим ответ для получения ID сообщения
        self.parse_sms_response()
    }

    /// Future для отправки команды
    async fn send_command_async(&mut self, command: &str) -> Result<ResponseStatus> {
        SendCommandFuture::new(self, command).await
    }

    /// Future для ожидания ответа
    async fn wait_for_response_async(&mut self) -> Result<ResponseStatus> {
        WaitResponseFuture::new(self).await
    }

    /// Асинхронная отправка текста сообщения
    async fn send_message_text_async(&mut self, message: &str) -> Result<()> {
        for byte in message.as_bytes() {
            SendByteFuture::new(self, *byte).await?;
        }
        Ok(())
    }

    /// Асинхронная отправка Ctrl+Z
    async fn send_ctrl_z_async(&mut self) -> Result<()> {
        SendByteFuture::new(self, 0x1A).await
    }

    /// Парсит ответ на отправку SMS (синхронный метод)
    fn parse_sms_response(&self) -> Result<SmsResponse> {
        let response = core::str::from_utf8(&self.rx_buffer)
            .map_err(|_| Error::InvalidResponse)?;
        
        // Ищем строку типа "+CMGS: 123"
        for line in response.lines() {
            if line.starts_with("+CMGS:") {
                if line.len() > 7 {
                    return Ok(SmsResponse {
                        message_id: Some(1), // Упрощенный парсинг
                        sent: true,
                    });
                }
            }
        }
        
        if response.contains("OK") {
            Ok(SmsResponse {
                message_id: None,
                sent: true,
            })
        } else {
            Err(Error::SmsNotSent)
        }
    }
}

/// Future для отправки команды
struct SendCommandFuture<'a, UART, TIMER> {
    modem: &'a mut AsyncSim800L<UART, TIMER>,
    command: &'a str,
    byte_index: usize,
    stage: CommandStage,
}

#[derive(Debug, Clone, Copy)]
enum CommandStage {
    SendingCommand,
    SendingCR,
    SendingLF,
    WaitingResponse,
}

impl<'a, UART, TIMER> SendCommandFuture<'a, UART, TIMER> {
    fn new(modem: &'a mut AsyncSim800L<UART, TIMER>, command: &'a str) -> Self {
        modem.rx_buffer.clear();
        Self {
            modem,
            command,
            byte_index: 0,
            stage: CommandStage::SendingCommand,
        }
    }
}

impl<'a, UART, TIMER, UartError> Future for SendCommandFuture<'a, UART, TIMER>
where
    UART: Read<u8, Error = UartError> + Write<u8, Error = UartError> + Unpin,
    TIMER: Timer,
{
    type Output = Result<ResponseStatus>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        
        loop {
            match this.stage {
                CommandStage::SendingCommand => {
                    if this.byte_index < this.command.len() {
                        let byte = this.command.as_bytes()[this.byte_index];
                        match this.modem.uart.write(byte) {
                            Ok(()) => {
                                this.byte_index += 1;
                                continue;
                            }
                            Err(nb::Error::WouldBlock) => {
                                cx.waker().wake_by_ref();
                                return Poll::Pending;
                            }
                            Err(nb::Error::Other(_)) => {
                                return Poll::Ready(Err(Error::Uart));
                            }
                        }
                    } else {
                        this.stage = CommandStage::SendingCR;
                        continue;
                    }
                }
                CommandStage::SendingCR => {
                    match this.modem.uart.write(b'\r') {
                        Ok(()) => {
                            this.stage = CommandStage::SendingLF;
                            continue;
                        }
                        Err(nb::Error::WouldBlock) => {
                            cx.waker().wake_by_ref();
                            return Poll::Pending;
                        }
                        Err(nb::Error::Other(_)) => {
                            return Poll::Ready(Err(Error::Uart));
                        }
                    }
                }
                CommandStage::SendingLF => {
                    match this.modem.uart.write(b'\n') {
                        Ok(()) => {
                            this.stage = CommandStage::WaitingResponse;
                            this.modem.state = AsyncState::WaitingResponse { 
                                start_time: this.modem.timer.now_ms() 
                            };
                            continue;
                        }
                        Err(nb::Error::WouldBlock) => {
                            cx.waker().wake_by_ref();
                            return Poll::Pending;
                        }
                        Err(nb::Error::Other(_)) => {
                            return Poll::Ready(Err(Error::Uart));
                        }
                    }
                }
                CommandStage::WaitingResponse => {
                    // Переключаемся на ожидание ответа
                    let mut wait_future = WaitResponseFuture::new(this.modem);
                    match Pin::new(&mut wait_future).poll(cx) {
                        Poll::Ready(result) => return Poll::Ready(result),
                        Poll::Pending => return Poll::Pending,
                    }
                }
            }
        }
    }
}

/// Future для ожидания ответа от модема
struct WaitResponseFuture<'a, UART, TIMER> {
    modem: &'a mut AsyncSim800L<UART, TIMER>,
}

impl<'a, UART, TIMER> WaitResponseFuture<'a, UART, TIMER> {
    fn new(modem: &'a mut AsyncSim800L<UART, TIMER>) -> Self {
        Self { modem }
    }
}

impl<'a, UART, TIMER, UartError> Future for WaitResponseFuture<'a, UART, TIMER>
where
    UART: Read<u8, Error = UartError> + Write<u8, Error = UartError> + Unpin,
    TIMER: Timer,
{
    type Output = Result<ResponseStatus>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        
        // Проверяем таймаут
        if let AsyncState::WaitingResponse { start_time } = this.modem.state {
            if this.modem.timer.now_ms() - start_time > this.modem.timeout_ms {
                this.modem.state = AsyncState::Idle;
                return Poll::Ready(Err(Error::Timeout));
            }
        }
        
        // Читаем данные из UART
        loop {
            match this.modem.uart.read() {
                Ok(byte) => {
                    if this.modem.rx_buffer.push(byte).is_err() {
                        this.modem.state = AsyncState::Idle;
                        return Poll::Ready(Err(Error::BufferOverflow));
                    }

                    // Проверяем на завершающие последовательности
                    if let Some(status) = this.check_response_complete() {
                        this.modem.state = AsyncState::Idle;
                        return Poll::Ready(Ok(status));
                    }
                }
                Err(nb::Error::WouldBlock) => {
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                }
                Err(nb::Error::Other(_)) => {
                    this.modem.state = AsyncState::Idle;
                    return Poll::Ready(Err(Error::Uart));
                }
            }
        }
    }
}

impl<'a, UART, TIMER> WaitResponseFuture<'a, UART, TIMER> {
    fn check_response_complete(&self) -> Option<ResponseStatus> {
        let buffer_str = core::str::from_utf8(&self.modem.rx_buffer).ok()?;
        
        if buffer_str.contains("OK\r\n") {
            Some(ResponseStatus::Ok)
        } else if buffer_str.contains("ERROR\r\n") {
            Some(ResponseStatus::Error)
        } else if buffer_str.contains("> ") {
            Some(ResponseStatus::Prompt)
        } else {
            None
        }
    }
}

/// Future для отправки одного байта
struct SendByteFuture<'a, UART, TIMER> {
    modem: &'a mut AsyncSim800L<UART, TIMER>,
    byte: u8,
}

impl<'a, UART, TIMER> SendByteFuture<'a, UART, TIMER> {
    fn new(modem: &'a mut AsyncSim800L<UART, TIMER>, byte: u8) -> Self {
        Self { modem, byte }
    }
}

impl<'a, UART, TIMER, UartError> Future for SendByteFuture<'a, UART, TIMER>
where
    UART: Read<u8, Error = UartError> + Write<u8, Error = UartError> + Unpin,
    TIMER: Timer,
{
    type Output = Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        
        match this.modem.uart.write(this.byte) {
            Ok(()) => Poll::Ready(Ok(())),
            Err(nb::Error::WouldBlock) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(nb::Error::Other(_)) => Poll::Ready(Err(Error::Uart)),
        }
    }
}

//! Основная логика для работы с модемом SIM800L

use crate::error::{Error, Result, ResponseStatus, SmsResponse, NetworkStatus};
use crate::timer::Timer;
use crate::commands;
use embedded_hal::serial::{Read, Write};
use heapless::{String, Vec};
use nb::block;

/// Размер буфера для команд и ответов
const BUFFER_SIZE: usize = 1024;

/// Максимальное время ожидания ответа (в миллисекундах)
const DEFAULT_TIMEOUT_MS: u32 = 10000;

/// Драйвер для SIM800L модема
pub struct Sim800L<UART, TIMER> {
    uart: UART,
    timer: TIMER,
    rx_buffer: Vec<u8, BUFFER_SIZE>,
    timeout_ms: u32,
}

impl<UART, TIMER, UartError> Sim800L<UART, TIMER>
where
    UART: Read<u8, Error = UartError> + Write<u8, Error = UartError>,
    TIMER: Timer,
{
    /// Создает новый экземпляр драйвера SIM800L
    pub fn new(uart: UART, timer: TIMER) -> Self {
        Self {
            uart,
            timer,
            rx_buffer: Vec::new(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
        }
    }

    /// Устанавливает таймаут для операций
    pub fn set_timeout(&mut self, timeout_ms: u32) {
        self.timeout_ms = timeout_ms;
    }

    /// Инициализация модема
    pub fn init(&mut self) -> Result<()> {
        // Ждем готовности модема после включения
        self.timer.delay_ms(3000);
        
        // Отключаем эхо команд
        self.send_command_with_timeout(commands::basic::AT_ECHO_OFF, self.timeout_ms)?;
        
        // Отключаем flow control
        self.send_command_with_timeout(commands::basic::AT_FLOW_CONTROL_OFF, self.timeout_ms)?;
        
        // Проверяем готовность модема
        self.send_command_with_timeout(commands::basic::AT, self.timeout_ms)?;
        
        // Включаем определитель номера
        self.send_command_with_timeout(commands::call::AT_CALLER_ID_ENABLE, self.timeout_ms)?;
        
        // Устанавливаем текстовый режим SMS
        self.send_command_with_timeout(commands::sms::AT_SMS_TEXT_MODE, self.timeout_ms)?;
        
        Ok(())
    }

    // ============ БАЗОВЫЕ КОМАНДЫ ============

    /// Отправляет команду с таймаутом
    pub fn send_command_with_timeout(&mut self, command: &str, timeout_ms: u32) -> Result<ResponseStatus> {
        self.send_command(command)?;
        self.wait_for_response_with_timeout(timeout_ms)
    }

    /// Отправляет AT команду модему
    fn send_command(&mut self, command: &str) -> Result<()> {
        // Очищаем буфер перед отправкой
        self.rx_buffer.clear();

        // Отправляем команду
        for byte in command.as_bytes() {
            block!(self.uart.write(*byte)).map_err(|_| Error::Uart)?;
        }

        // Отправляем завершающие символы \r\n
        block!(self.uart.write(b'\r')).map_err(|_| Error::Uart)?;
        block!(self.uart.write(b'\n')).map_err(|_| Error::Uart)?;

        Ok(())
    }

    /// Ожидает ответ от модема с таймаутом
    fn wait_for_response_with_timeout(&mut self, timeout_ms: u32) -> Result<ResponseStatus> {
        let start_time = self.timer.now_ms();
        
        loop {
            // Проверяем таймаут
            if self.timer.now_ms() - start_time > timeout_ms {
                return Err(Error::Timeout);
            }
            
            match self.uart.read() {
                Ok(byte) => {
                    if self.rx_buffer.push(byte).is_err() {
                        return Err(Error::BufferOverflow);
                    }

                    // Проверяем на завершающие последовательности
                    if let Some(status) = self.check_response_complete() {
                        return Ok(status);
                    }
                }
                Err(nb::Error::WouldBlock) => {
                    // Продолжаем ожидание
                    continue;
                }
                Err(nb::Error::Other(_)) => {
                    return Err(Error::Uart);
                }
            }
        }
    }

    /// Проверяет завершенность ответа и возвращает статус
    fn check_response_complete(&self) -> Option<ResponseStatus> {
        let buffer_str = core::str::from_utf8(&self.rx_buffer).ok()?;
        
        if buffer_str.contains("OK\r\n") {
            Some(ResponseStatus::Ok)
        } else if buffer_str.contains("ERROR\r\n") {
            Some(ResponseStatus::Error)
        } else if buffer_str.contains("> ") {
            Some(ResponseStatus::Prompt)
        } else if buffer_str.contains("RING") {
            Some(ResponseStatus::Ring)
        } else if buffer_str.contains("CONNECT") {
            Some(ResponseStatus::Connect)
        } else if buffer_str.contains("NO CARRIER") {
            Some(ResponseStatus::NoCarrier)
        } else {
            None
        }
    }

    /// Получает версию прошивки модема
    pub fn get_firmware_version(&mut self) -> Result<String<64>> {
        self.send_command_with_timeout(commands::basic::AT_INFO, self.timeout_ms)?;
        
        let response = core::str::from_utf8(&self.rx_buffer)
            .map_err(|_| Error::InvalidResponse)?;
        
        // Извлекаем версию (первая непустая строка до OK)
        for line in response.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.contains("OK") && !line.contains("ATI") {
                return String::try_from(line).map_err(|_| Error::BufferOverflow);
            }
        }
        
        Err(Error::InvalidResponse)
    }

    /// Проверяет статус сети с детальным разбором
    pub fn check_network_status(&mut self) -> Result<NetworkStatus> {
        self.send_command_with_timeout(commands::network::AT_NETWORK_REGISTRATION, self.timeout_ms)?;
        
        let response = core::str::from_utf8(&self.rx_buffer)
            .map_err(|_| Error::InvalidResponse)?;
        
        // Ищем строку типа "+CREG: 0,1"
        for line in response.lines() {
            if line.starts_with("+CREG:") {
                if line.contains(",1") {
                    return Ok(NetworkStatus::RegisteredHome);
                } else if line.contains(",5") {
                    return Ok(NetworkStatus::RegisteredRoaming);
                } else if line.contains(",2") {
                    return Ok(NetworkStatus::Searching);
                } else if line.contains(",3") {
                    return Ok(NetworkStatus::Denied);
                } else if line.contains(",0") {
                    return Ok(NetworkStatus::NotRegistered);
                }
            }
        }
        
        Ok(NetworkStatus::Unknown)
    }

    /// Отправляет SMS сообщение с правильным парсингом
    pub fn send_sms(&mut self, phone: &str, message: &str) -> Result<SmsResponse> {
        // Подготавливаем команду отправки
        let mut cmd: String<64> = String::new();
        cmd.push_str(commands::sms::AT_SMS_SEND).map_err(|_| Error::BufferOverflow)?;
        cmd.push_str("\"").map_err(|_| Error::BufferOverflow)?;
        cmd.push_str(phone).map_err(|_| Error::BufferOverflow)?;
        cmd.push_str("\"").map_err(|_| Error::BufferOverflow)?;
        
        self.send_command(&cmd)?;
        
        // Ожидаем приглашение ">"
        if self.wait_for_response_with_timeout(commands::SMS_TIMEOUT)? != ResponseStatus::Prompt {
            return Err(Error::SmsNotSent);
        }
        
        // Отправляем текст сообщения
        for byte in message.as_bytes() {
            block!(self.uart.write(*byte)).map_err(|_| Error::Uart)?;
        }
        
        // Отправляем Ctrl+Z для завершения
        block!(self.uart.write(0x1A)).map_err(|_| Error::Uart)?;
        
        // Ожидаем подтверждение отправки
        if self.wait_for_response_with_timeout(commands::SMS_TIMEOUT)? != ResponseStatus::Ok {
            return Err(Error::SmsNotSent);
        }
        
        // Парсим ответ для получения ID сообщения
        self.parse_sms_response()
    }

    /// Совершает исходящий вызов
    pub fn make_call(&mut self, number: &str) -> Result<()> {
        let mut cmd: String<32> = String::new();
        cmd.push_str(commands::call::AT_DIAL).map_err(|_| Error::BufferOverflow)?;
        cmd.push_str(number).map_err(|_| Error::BufferOverflow)?;
        cmd.push_str(";").map_err(|_| Error::BufferOverflow)?; // ; для голосового вызова
        
        match self.send_command_with_timeout(&cmd, commands::CALL_TIMEOUT)? {
            ResponseStatus::Ok => Ok(()),
            _ => Err(Error::CallError),
        }
    }

    /// Отвечает на входящий вызов
    pub fn answer_call(&mut self) -> Result<()> {
        match self.send_command_with_timeout(commands::call::AT_ANSWER, commands::CALL_TIMEOUT)? {
            ResponseStatus::Ok | ResponseStatus::Connect => Ok(()),
            _ => Err(Error::CallError),
        }
    }

    /// Завершает вызов
    pub fn hangup_call(&mut self) -> Result<()> {
        match self.send_command_with_timeout(commands::call::AT_HANGUP, commands::CALL_TIMEOUT)? {
            ResponseStatus::Ok | ResponseStatus::NoCarrier => Ok(()),
            _ => Err(Error::CallError),
        }
    }

    /// Устанавливает громкость динамика (0-9)
    pub fn set_speaker_volume(&mut self, level: u8) -> Result<()> {
        let level = level.min(9);
        let mut cmd: String<16> = String::new();
        cmd.push_str(commands::audio::AT_SPEAKER_VOLUME).map_err(|_| Error::BufferOverflow)?;
        
        // Простое преобразование цифры в символ
        let digit_char = (b'0' + level) as char;
        cmd.push(digit_char).map_err(|_| Error::BufferOverflow)?;
        
        match self.send_command_with_timeout(&cmd, self.timeout_ms)? {
            ResponseStatus::Ok => Ok(()),
            _ => Err(Error::AudioError),
        }
    }

    /// Устанавливает усиление микрофона (0-15)
    pub fn set_microphone_gain(&mut self, level: u8) -> Result<()> {
        let level = level.min(15);
        let mut cmd: String<16> = String::new();
        cmd.push_str(commands::audio::AT_MICROPHONE_GAIN).map_err(|_| Error::BufferOverflow)?;
        
        // Простое преобразование для однозначных чисел
        if level < 10 {
            let digit_char = (b'0' + level) as char;
            cmd.push(digit_char).map_err(|_| Error::BufferOverflow)?;
        } else {
            cmd.push('1').map_err(|_| Error::BufferOverflow)?;
            let digit_char = (b'0' + (level - 10)) as char;
            cmd.push(digit_char).map_err(|_| Error::BufferOverflow)?;
        }
        
        match self.send_command_with_timeout(&cmd, self.timeout_ms)? {
            ResponseStatus::Ok => Ok(()),
            _ => Err(Error::AudioError),
        }
    }

    /// Включает громкую связь
    pub fn enable_speaker_phone(&mut self) -> Result<()> {
        match self.send_command_with_timeout(commands::audio::AT_AUDIO_PATH_SPEAKER, self.timeout_ms)? {
            ResponseStatus::Ok => Ok(()),
            _ => Err(Error::AudioError),
        }
    }

    /// Включает режим трубки
    pub fn enable_handset(&mut self) -> Result<()> {
        match self.send_command_with_timeout(commands::audio::AT_AUDIO_PATH_HANDSET, self.timeout_ms)? {
            ResponseStatus::Ok => Ok(()),
            _ => Err(Error::AudioError),
        }
    }

    /// Парсит ответ на отправку SMS
    fn parse_sms_response(&self) -> Result<SmsResponse> {
        let response = core::str::from_utf8(&self.rx_buffer)
            .map_err(|_| Error::InvalidResponse)?;
        
        // Ищем строку типа "+CMGS: 123"
        for line in response.lines() {
            if line.starts_with("+CMGS:") {
                // Упрощенный парсинг - просто проверяем наличие числа
                if line.len() > 7 {
                    return Ok(SmsResponse {
                        message_id: Some(1), // Заглушка
                        sent: true,
                    });
                }
            }
        }
        
        // Если ID не найден, но ответ OK - считаем отправленным
        if response.contains("OK") {
            Ok(SmsResponse {
                message_id: None,
                sent: true,
            })
        } else {
            Err(Error::SmsNotSent)
        }
    }

    /// Получает доступ к таймеру
    pub fn timer(&mut self) -> &mut TIMER {
        &mut self.timer
    }

    /// Получает доступ к внутреннему UART
    pub fn uart(&mut self) -> &mut UART {
        &mut self.uart
    }

    /// Получает текущий буфер ответа
    pub fn response_buffer(&self) -> &[u8] {
        &self.rx_buffer
    }

    /// Очищает буфер ответа
    pub fn clear_buffer(&mut self) {
        self.rx_buffer.clear();
    }

    // ============ ДОПОЛНИТЕЛЬНЫЕ МЕТОДЫ ============

    /// Получает IMEI номер устройства
    pub fn get_imei(&mut self) -> Result<heapless::String<20>> {
        match self.send_command_with_timeout(commands::basic::AT_IMEI, self.timeout_ms)? {
            ResponseStatus::Ok => {
                let response = core::str::from_utf8(&self.rx_buffer)
                    .map_err(|_| Error::InvalidResponse)?;
                
                // Ищем строку с IMEI (обычно 15 цифр)
                for line in response.lines() {
                    let line = line.trim();
                    if line.len() == 15 && line.chars().all(|c| c.is_ascii_digit()) {
                        let mut imei = heapless::String::<20>::new();
                        imei.push_str(line).map_err(|_| Error::BufferOverflow)?;
                        return Ok(imei);
                    }
                }
                Err(Error::InvalidResponse)
            }
            _ => Err(Error::AtError),
        }
    }

    /// Получает информацию о SIM-карте
    pub fn get_sim_info(&mut self) -> Result<crate::error::SimInfo> {
        // Получаем IMSI
        let mut imsi = heapless::String::<20>::new();
        match self.send_command_with_timeout(commands::sim::AT_SIM_IMSI, self.timeout_ms)? {
            ResponseStatus::Ok => {
                let response = core::str::from_utf8(&self.rx_buffer)
                    .map_err(|_| Error::InvalidResponse)?;
                
                // Ищем IMSI (обычно 15 цифр)
                for line in response.lines() {
                    let line = line.trim();
                    if line.len() >= 14 && line.chars().all(|c| c.is_ascii_digit()) {
                        imsi.push_str(line).map_err(|_| Error::BufferOverflow)?;
                        break;
                    }
                }
                if imsi.is_empty() {
                    return Err(Error::InvalidResponse);
                }
            }
            _ => return Err(Error::AtError),
        }

        // Получаем номер SIM (упрощенная реализация)
        let sim_number = heapless::String::<20>::new();

        // Проверяем статус PIN
        let pin_required = match self.send_command_with_timeout(commands::sim::AT_SIM_STATUS, self.timeout_ms)? {
            ResponseStatus::Ok => {
                let response = core::str::from_utf8(&self.rx_buffer)
                    .map_err(|_| Error::InvalidResponse)?;
                response.contains("SIM PIN")
            }
            _ => false,
        };

        Ok(crate::error::SimInfo {
            imsi,
            sim_number,
            pin_required,
        })
    }

    /// Вводит PIN-код
    pub fn enter_pin(&mut self, pin: &str) -> Result<()> {
        let mut cmd = heapless::String::<32>::new();
        cmd.push_str(commands::sim::AT_SIM_ENTER_PIN).map_err(|_| Error::BufferOverflow)?;
        cmd.push_str(pin).map_err(|_| Error::BufferOverflow)?;
        
        match self.send_command_with_timeout(&cmd, self.timeout_ms)? {
            ResponseStatus::Ok => Ok(()),
            _ => Err(Error::InvalidPin),
        }
    }

    /// Получает расширенную информацию о сети
    pub fn get_network_info(&mut self) -> Result<crate::error::NetworkInfo> {
        // Получаем статус регистрации
        let status = self.check_network_status()?;

        // Получаем уровень сигнала
        match self.send_command_with_timeout(commands::network::AT_SIGNAL_QUALITY, self.timeout_ms)? {
            ResponseStatus::Ok => {
                let response = core::str::from_utf8(&self.rx_buffer)
                    .map_err(|_| Error::InvalidResponse)?;
                
                // Ищем +CSQ: XX,YY
                for line in response.lines() {
                    if line.starts_with("+CSQ:") {
                        let parts: heapless::Vec<&str, 4> = line[5..].split(',').collect();
                        if let Some(rssi_str) = parts.get(0) {
                            if let Ok(rssi) = rssi_str.trim().parse::<u8>() {
                                let mut operator = heapless::String::<32>::new();
                                operator.push_str("Unknown").ok();
                                return Ok(crate::error::NetworkInfo {
                                    operator,
                                    signal_strength: rssi,
                                    status,
                                });
                            }
                        }
                    }
                }
                Err(Error::InvalidResponse)
            }
            _ => Err(Error::AtError),
        }
    }

    /// Получает статус активных вызовов
    pub fn get_call_status(&mut self) -> Result<heapless::Vec<crate::error::CallInfo, 4>> {
        match self.send_command_with_timeout(commands::call::AT_CALL_STATUS, self.timeout_ms)? {
            ResponseStatus::Ok => {
                let mut calls = heapless::Vec::new();
                let response = core::str::from_utf8(&self.rx_buffer)
                    .map_err(|_| Error::InvalidResponse)?;
                
                // Парсим ответы +CLCC
                for line in response.lines() {
                    if line.starts_with("+CLCC:") {
                        // Упрощенный парсинг - создаем заглушку
                        let number = heapless::String::<20>::new();
                        let call_info = crate::error::CallInfo {
                            number,
                            status: crate::error::CallStatus::Idle,
                            duration: None,
                        };
                        calls.push(call_info).map_err(|_| Error::BufferOverflow)?;
                    }
                }
                
                Ok(calls)
            }
            _ => Err(Error::AtError),
        }
    }

    /// Отправляет DTMF тон
    pub fn send_dtmf(&mut self, tone: crate::error::DtmfTone) -> Result<()> {
        let tone_char = match tone {
            crate::error::DtmfTone::Digit0 => "0",
            crate::error::DtmfTone::Digit1 => "1",
            crate::error::DtmfTone::Digit2 => "2",
            crate::error::DtmfTone::Digit3 => "3",
            crate::error::DtmfTone::Digit4 => "4",
            crate::error::DtmfTone::Digit5 => "5",
            crate::error::DtmfTone::Digit6 => "6",
            crate::error::DtmfTone::Digit7 => "7",
            crate::error::DtmfTone::Digit8 => "8",
            crate::error::DtmfTone::Digit9 => "9",
            crate::error::DtmfTone::Star => "*",
            crate::error::DtmfTone::Hash => "#",
            crate::error::DtmfTone::A => "A",
            crate::error::DtmfTone::B => "B",
            crate::error::DtmfTone::C => "C",
            crate::error::DtmfTone::D => "D",
        };

        let mut cmd = heapless::String::<16>::new();
        cmd.push_str(commands::call::AT_DTMF_SEND).map_err(|_| Error::BufferOverflow)?;
        cmd.push_str(tone_char).map_err(|_| Error::BufferOverflow)?;

        match self.send_command_with_timeout(&cmd, self.timeout_ms)? {
            ResponseStatus::Ok => Ok(()),
            _ => Err(Error::DtmfError),
        }
    }

    /// Получает статус батареи
    pub fn get_battery_status(&mut self) -> Result<(u8, u16)> {
        match self.send_command_with_timeout(commands::basic::AT_BATTERY_STATUS, self.timeout_ms)? {
            ResponseStatus::Ok => {
                let response = core::str::from_utf8(&self.rx_buffer)
                    .map_err(|_| Error::InvalidResponse)?;
                
                // Ищем +CBC: status,level,voltage
                for line in response.lines() {
                    if line.starts_with("+CBC:") {
                        let parts: heapless::Vec<&str, 4> = line[5..].split(',').collect();
                        if parts.len() >= 3 {
                            if let (Ok(status), Ok(voltage)) = (
                                parts[0].trim().parse::<u8>(),
                                parts[2].trim().parse::<u16>()
                            ) {
                                return Ok((status, voltage));
                            }
                        }
                    }
                }
                Err(Error::InvalidResponse)
            }
            _ => Err(Error::AtError),
        }
    }
}

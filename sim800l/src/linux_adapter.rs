//! Linux адаптер для использования SIM800L на Raspberry Pi Zero 2W
//! 
//! Этот модуль предоставляет wrapper для использования библиотеки SIM800L
//! в Linux окружении с serial port API

#[cfg(feature = "std")]
use std::{
    io::{Read, Write},
    time::{Duration, Instant},
};

#[cfg(feature = "std")]
use serialport::{SerialPort, Error as SerialError};

/// Linux адаптер для UART, совместимый с embedded-hal
#[cfg(feature = "std")]
pub struct LinuxUart {
    port: Box<dyn SerialPort>,
}

#[cfg(feature = "std")]
impl LinuxUart {
    /// Создает новый LinuxUart для работы с SIM800L на Raspberry Pi
    pub fn new(device: &str) -> Result<Self, SerialError> {
        let port = serialport::new(device, 115200)
            .timeout(Duration::from_millis(100))
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .parity(serialport::Parity::None)
            .flow_control(serialport::FlowControl::None)
            .open()?;

        Ok(Self { port })
    }
}

#[cfg(feature = "std")]
impl embedded_hal::serial::Read<u8> for LinuxUart {
    type Error = SerialError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buf = [0u8; 1];
        match self.port.read_exact(&mut buf) {
            Ok(()) => Ok(buf[0]),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => Err(nb::Error::WouldBlock),
            Err(e) => Err(nb::Error::Other(SerialError::from(e))),
        }
    }
}

#[cfg(feature = "std")]
impl embedded_hal::serial::Write<u8> for LinuxUart {
    type Error = SerialError;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        match self.port.write_all(&[byte]) {
            Ok(()) => Ok(()),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => Err(nb::Error::WouldBlock),
            Err(e) => Err(nb::Error::Other(SerialError::from(e))),
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        match self.port.flush() {
            Ok(()) => Ok(()),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => Err(nb::Error::WouldBlock),
            Err(e) => Err(nb::Error::Other(SerialError::from(e))),
        }
    }
}

/// Linux таймер, совместимый с нашим Timer trait
#[cfg(feature = "std")]
pub struct LinuxTimer {
    start_time: Instant,
}

#[cfg(feature = "std")]
impl LinuxTimer {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
}

#[cfg(feature = "std")]
impl crate::timer::Timer for LinuxTimer {
    fn delay_ms(&mut self, ms: u32) {
        std::thread::sleep(Duration::from_millis(ms as u64));
    }

    fn now_ms(&self) -> u32 {
        self.start_time.elapsed().as_millis() as u32
    }
}

/// Пример использования на Raspberry Pi Zero 2W
#[cfg(feature = "std")]
pub fn raspberry_pi_example() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{Sim800L, AsyncSim800L};

    // Подключение к SIM800L через GPIO UART
    let uart = LinuxUart::new("/dev/ttyS0")?; // или /dev/ttyAMA0
    let timer = LinuxTimer::new();

    // Синхронное использование
    let mut modem = Sim800L::new(uart, timer);
    modem.init()?;
    
    println!("📱 SIM800L готов на Raspberry Pi Zero 2W!");
    
    // Отправка SMS
    modem.send_sms("+1234567890", "Hello from Pi Zero 2W!")?;
    
    println!("✅ SMS отправлен!");

    Ok(())
}

/// Асинхронный пример для Raspberry Pi с Tokio
#[cfg(all(feature = "std", feature = "tokio"))]
pub async fn raspberry_pi_async_example() -> Result<(), Box<dyn std::error::Error>> {
    use crate::AsyncSim800L;

    let uart = LinuxUart::new("/dev/ttyS0")?;
    let timer = LinuxTimer::new();

    let mut modem = AsyncSim800L::new(uart, timer);
    
    // Асинхронная инициализация
    modem.init().await?;
    
    println!("📱 Асинхронный SIM800L готов!");
    
    // Параллельные операции на Pi!
    let (call_result, sms_result, battery_info) = tokio::join!(
        modem.make_call_async("+1234567890"),
        modem.send_sms_async("+0987654321", "Async Pi Zero 2W!"),
        modem.get_battery_status_async()
    );
    
    println!("🚀 Все операции выполнены параллельно!");

    Ok(())
}

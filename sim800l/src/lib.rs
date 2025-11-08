//! # SIM800L Library
//! 
//! Полнофункциональная библиотека для работы с GSM/GPRS модемом SIM800L в embedded системах.
//! Поддерживает голосовые вызовы, SMS, аудио управление и все основные функции телефона.
//! Доступны синхронный и асинхронный API для максимальной гибкости.
//! 
//! ## Особенности
//! 
//! - `#![no_std]` совместимость
//! - Поддержка embedded-hal для UART
//! - Скорость UART 115200 бод
//! - Без Flow Control (AT+IFC=0,0)
//! - **Синхронный и асинхронный API**
//! - Полный функционал телефона:
//!   - Голосовые вызовы
//!   - SMS отправка/получение
//!   - Управление аудио
//!   - Работа с SIM-картой
//!   - Сетевые функции
//!   - DTMF тоны
//! 
//! ## Пример использования (синхронный API)
//! 
//! ```rust,no_run
//! use sim800l::{Sim800L, MockTimer};
//! 
//! // uart и timer должны быть настроены для вашей платформы
//! let mut modem = Sim800L::new(uart, timer);
//! 
//! // Инициализация модема
//! modem.init().unwrap();
//! 
//! // Совершение вызова
//! modem.make_call("+1234567890").unwrap();
//! 
//! // Отправка SMS
//! let response = modem.send_sms("+1234567890", "Hello!").unwrap();
//! ```
//! 
//! ## Пример использования (асинхронный API)
//! 
//! ```rust,no_run
//! use sim800l::{AsyncSim800L, MockTimer};
//! 
//! // uart и timer должны быть настроены для вашей платформы
//! let mut async_modem = AsyncSim800L::new(uart, timer);
//! 
//! // Инициализация модема асинхронно
//! async_modem.init().await.unwrap();
//! 
//! // Отправка SMS асинхронно (не блокирует CPU)
//! let response = async_modem.send_sms_async("+1234567890", "Hello!").await.unwrap();
//! ```

#![no_std]
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod modem;
pub mod async_modem;
pub mod timer;
pub mod commands;

// Linux адаптер для std окружений
#[cfg(feature = "linux")]
pub mod linux_adapter;

// Реэкспорт основных типов
pub use error::{
    Error, Result, ResponseStatus, SmsResponse, IncomingSms,
    NetworkStatus, NetworkInfo, CallInfo, CallStatus, 
    SimInfo, VolumeLevel, DtmfTone
};
pub use modem::Sim800L;
pub use async_modem::AsyncSim800L;
pub use timer::{Timer, MockTimer};

// Linux адаптеры для Raspberry Pi
#[cfg(feature = "linux")]
pub use linux_adapter::{LinuxUart, LinuxTimer};

/// Версия библиотеки
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Рекомендуемые настройки UART
pub mod uart_config {
    /// Рекомендуемая скорость UART (бод)
    pub const BAUD_RATE: u32 = 115200;
    
    /// Количество бит данных
    pub const DATA_BITS: u8 = 8;
    
    /// Четность отключена
    pub const PARITY: &str = "None";
    
    /// Количество стоп-бит
    pub const STOP_BITS: u8 = 1;
    
    /// Flow control отключен
    pub const FLOW_CONTROL: bool = false;
}

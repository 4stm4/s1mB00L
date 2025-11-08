//! AT команды для SIM800L телефона

pub mod basic;
pub mod sms;
pub mod network;
pub mod call;
pub mod audio;
pub mod sim;

// Константы таймаутов
/// Стандартный таймаут для AT команд (5 секунд)
pub const AT_TIMEOUT: u32 = 5000;       // 5 секунд
/// Таймаут для SMS операций (30 секунд)
pub const SMS_TIMEOUT: u32 = 30000;     // 30 секунд
/// Таймаут для сетевых операций (15 секунд)
pub const NETWORK_TIMEOUT: u32 = 15000; // 15 секунд
/// Таймаут для голосовых вызовов (60 секунд)
pub const CALL_TIMEOUT: u32 = 60000;    // 60 секунд
/// Таймаут для операций с SIM-картой (10 секунд)
pub const SIM_TIMEOUT: u32 = 10000;     // 10 секунд

//! Команды для работы с голосовыми вызовами

/// Команды вызовов
/// Набрать номер (использовать как ATD+1234567890;)
pub const AT_DIAL: &str = "ATD";           // ATD+1234567890; - набрать номер
/// Ответить на входящий вызов
pub const AT_ANSWER: &str = "ATA";         // Ответить на вызов
/// Завершить текущий вызов
pub const AT_HANGUP: &str = "ATH";         // Завершить вызов
/// Получить статус всех текущих вызовов
pub const AT_CALL_STATUS: &str = "AT+CLCC"; // Статус текущих вызовов

/// Настройки вызовов
/// Включить определитель номера
pub const AT_CALLER_ID_ENABLE: &str = "AT+CLIP=1";  // Включить определитель номера
/// Отключить определитель номера
pub const AT_CALLER_ID_DISABLE: &str = "AT+CLIP=0"; // Выключить определитель номера
/// Включить ожидание вызова
pub const AT_CALL_WAITING_ENABLE: &str = "AT+CCWA=1"; // Включить ожидание вызова
/// Отключить ожидание вызова
pub const AT_CALL_WAITING_DISABLE: &str = "AT+CCWA=0"; // Выключить ожидание вызова

/// DTMF команды
/// Отправить DTMF тон (например AT+VTS=1)
pub const AT_DTMF_SEND: &str = "AT+VTS=";  // AT+VTS=1 - отправить DTMF тон
/// Установить длительность DTMF тона
pub const AT_DTMF_DURATION: &str = "AT+VTD="; // Длительность DTMF тона

/// Команды переадресации
/// Безусловная переадресация вызовов
pub const AT_CALL_FORWARD_UNCONDITIONAL: &str = "AT+CCFC=0"; // Безусловная переадресация
/// Переадресация при занятости
pub const AT_CALL_FORWARD_BUSY: &str = "AT+CCFC=1";          // При занятости
/// Переадресация при неответе
pub const AT_CALL_FORWARD_NO_REPLY: &str = "AT+CCFC=2";      // При неответе
/// Переадресация при недоступности
pub const AT_CALL_FORWARD_UNREACHABLE: &str = "AT+CCFC=3";   // При недоступности

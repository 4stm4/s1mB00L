//! Команды для работы с сетью

/// Регистрация в сети
/// Проверить статус регистрации в сети
pub const AT_NETWORK_REGISTRATION: &str = "AT+CREG?"; // Статус регистрации
/// Автоматический выбор сети
pub const AT_NETWORK_SELECTION_AUTO: &str = "AT+COPS=0"; // Автовыбор сети
/// Ручной выбор сети
pub const AT_NETWORK_SELECTION_MANUAL: &str = "AT+COPS=1"; // Ручной выбор
/// Получить список доступных операторов
pub const AT_NETWORK_OPERATORS: &str = "AT+COPS=?";   // Доступные операторы
/// Получить текущего оператора
pub const AT_NETWORK_CURRENT: &str = "AT+COPS?";      // Текущий оператор

/// Качество сигнала
/// Получить качество сигнала
pub const AT_SIGNAL_QUALITY: &str = "AT+CSQ";         // Уровень сигнала
/// Получить расширенную информацию о сигнале
pub const AT_SIGNAL_EXTENDED: &str = "AT+CESQ";       // Расширенная информация

/// Время и дата
/// Получить время и дату
pub const AT_CLOCK_GET: &str = "AT+CCLK?";            // Получить время
/// Установить время и дату
pub const AT_CLOCK_SET: &str = "AT+CCLK=";            // Установить время

/// GPRS команды
/// Подключиться к GPRS
pub const AT_GPRS_ATTACH: &str = "AT+CGATT=1";        // Подключиться к GPRS
/// Отключиться от GPRS
pub const AT_GPRS_DETACH: &str = "AT+CGATT=0";        // Отключиться от GPRS
/// Получить статус GPRS
pub const AT_GPRS_STATUS: &str = "AT+CGATT?";         // Статус GPRS
/// Установить параметры APN
pub const AT_APN_SET: &str = "AT+CSTT=";              // Установить APN

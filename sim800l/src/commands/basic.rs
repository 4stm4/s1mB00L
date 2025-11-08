//! Базовые AT команды

/// Базовые AT команды
pub const AT: &str = "AT";
/// Получить информацию о модеме
pub const AT_INFO: &str = "ATI";
/// Отключить эхо команд
pub const AT_ECHO_OFF: &str = "ATE0";
/// Включить эхо команд
pub const AT_ECHO_ON: &str = "ATE1";
/// Сброс к заводским настройкам
pub const AT_FACTORY_RESET: &str = "AT&F";
/// Сохранить текущую конфигурацию
pub const AT_SAVE_CONFIG: &str = "AT&W";
/// Отключить управление потоком
pub const AT_FLOW_CONTROL_OFF: &str = "AT+IFC=0,0";

/// Команды управления питанием
/// Выключить модем
pub const AT_POWER_DOWN: &str = "AT+CPOWD=1";
/// Включить режим сна
pub const AT_SLEEP_MODE: &str = "AT+CSCLK=1";
/// Выйти из режима сна
pub const AT_WAKE_UP: &str = "AT+CSCLK=0";

/// Команды информации
/// Получить IMEI номер
pub const AT_IMEI: &str = "AT+GSN";
/// Получить производителя
pub const AT_MANUFACTURER: &str = "AT+CGMI";
/// Получить модель устройства
pub const AT_MODEL: &str = "AT+CGMM";
/// Получить версию прошивки
pub const AT_REVISION: &str = "AT+CGMR";
/// Получить статус батареи
pub const AT_BATTERY_STATUS: &str = "AT+CBC";

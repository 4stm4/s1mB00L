//! Команды для работы с SIM-картой

/// Статус SIM-карты
/// Проверить статус PIN кода
pub const AT_SIM_STATUS: &str = "AT+CPIN?";         // Статус PIN
/// Ввести PIN код
pub const AT_SIM_ENTER_PIN: &str = "AT+CPIN=";      // Ввести PIN
/// Сменить PIN код
pub const AT_SIM_CHANGE_PIN: &str = "AT+CPWD=";     // Сменить PIN

/// Информация о SIM
/// Получить IMSI номер
pub const AT_SIM_IMSI: &str = "AT+CIMI";            // IMSI номер
/// Получить номер SIM-карты
pub const AT_SIM_PHONE_NUMBER: &str = "AT+CNUM";    // Номер SIM-карты
/// Получить имя провайдера
pub const AT_SIM_PROVIDER: &str = "AT+CSPN?";       // Провайдер

/// Телефонная книга
/// Выбрать телефонную книгу
pub const AT_PHONEBOOK_SELECT: &str = "AT+CPBS=";   // Выбрать книгу
/// Прочитать запись из телефонной книги
pub const AT_PHONEBOOK_READ: &str = "AT+CPBR=";     // Читать запись
/// Записать контакт в телефонную книгу
pub const AT_PHONEBOOK_WRITE: &str = "AT+CPBW=";    // Записать контакт
/// Найти контакт в телефонной книге
pub const AT_PHONEBOOK_FIND: &str = "AT+CPBF=";     // Найти контакт

/// Услуги сети
/// Отправить USSD запрос
pub const AT_USSD_SEND: &str = "AT+CUSD=";          // Отправить USSD
/// Настроить блокировку вызовов
pub const AT_CALL_BARRING: &str = "AT+CLCK=";       // Блокировка вызовов

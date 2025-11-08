//! Команды для работы с SMS

/// Установить текстовый режим SMS
pub const AT_SMS_TEXT_MODE: &str = "AT+CMGF=1";     // Текстовый режим
/// Установить PDU режим SMS
pub const AT_SMS_PDU_MODE: &str = "AT+CMGF=0";      // PDU режим
/// Отправить SMS сообщение
pub const AT_SMS_SEND: &str = "AT+CMGS=";           // Отправить SMS
/// Прочитать SMS сообщение
pub const AT_SMS_READ: &str = "AT+CMGR=";           // Прочитать SMS
/// Получить список всех SMS сообщений
pub const AT_SMS_LIST_ALL: &str = "AT+CMGL=\"ALL\""; // Список всех SMS
/// Получить список непрочитанных SMS
pub const AT_SMS_LIST_UNREAD: &str = "AT+CMGL=\"REC UNREAD\""; // Непрочитанные
/// Удалить SMS сообщение
pub const AT_SMS_DELETE: &str = "AT+CMGD=";         // Удалить SMS

/// Настройки SMS
/// Выбрать хранилище SMS
pub const AT_SMS_STORAGE_SELECT: &str = "AT+CPMS="; // Выбор хранилища
/// Настроить индикацию новых SMS
pub const AT_SMS_NEW_MESSAGE_IND: &str = "AT+CNMI="; // Индикация новых SMS
/// Установить кодировку символов
pub const AT_SMS_CHARACTER_SET: &str = "AT+CSCS=";   // Кодировка

/// Центр SMS сообщений
/// Получить/установить адрес SMS центра
pub const AT_SMS_CENTER_ADDRESS: &str = "AT+CSCA"; // Адрес SMS центра

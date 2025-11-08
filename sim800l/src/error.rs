//! Типы ошибок для библиотеки SIM800L

use heapless::String;

/// Результат операции с SIM800L
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Типы ошибок SIM800L
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    /// Ошибка UART
    Uart,
    /// Переполнение буфера
    BufferOverflow,
    /// Модем не готов
    ModemNotReady,
    /// Неверный ответ от модема
    InvalidResponse,
    /// Ошибка выполнения AT команды
    AtError,
    /// Таймаут операции
    Timeout,
    /// Сеть недоступна
    NetworkNotAvailable,
    /// SMS не отправлено
    SmsNotSent,
    /// Неподдерживаемая операция
    NotSupported,
    /// Ошибка вызова
    CallError,
    /// Номер занят
    Busy,
    /// Нет ответа
    NoAnswer,
    /// SIM-карта не найдена
    SimNotFound,
    /// Неверный PIN-код
    InvalidPin,
    /// Блокировка PIN
    PinBlocked,
    /// Ошибка аудио
    AudioError,
    /// Ошибка DTMF
    DtmfError,
    /// Низкий заряд батареи
    LowBattery,
}

/// Статус ответа от модема
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResponseStatus {
    /// Команда выполнена успешно
    Ok,
    /// Ошибка выполнения команды
    Error,
    /// Ожидание завершения
    Pending,
    /// Приглашение к вводу (например, ">" для SMS)
    Prompt,
    /// Входящий вызов
    Ring,
    /// Соединение установлено
    Connect,
    /// Нет несущей
    NoCarrier,
}

/// Результат парсинга SMS ответа
#[derive(Debug, Clone, PartialEq)]
pub struct SmsResponse {
    /// Индекс сообщения в памяти модема
    pub message_id: Option<u16>,
    /// Статус отправки
    pub sent: bool,
}

/// Информация о входящем SMS
#[derive(Debug, Clone, PartialEq)]
pub struct IncomingSms {
    /// Индекс в памяти
    pub index: u16,
    /// Номер отправителя
    pub sender: String<20>,
    /// Время получения
    pub timestamp: String<20>,
    /// Текст сообщения
    pub text: String<160>,
}

/// Статус регистрации в сети
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkStatus {
    /// Не зарегистрирован, поиск не ведется
    NotRegistered,
    /// Зарегистрирован в домашней сети
    RegisteredHome,
    /// Не зарегистрирован, ведется поиск
    Searching,
    /// Регистрация отклонена
    Denied,
    /// Неизвестный статус
    Unknown,
    /// Зарегистрирован в роуминге
    RegisteredRoaming,
}

/// Информация о сети
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInfo {
    /// Оператор сети
    pub operator: String<32>,
    /// Уровень сигнала (0-31, 99 = неизвестно)
    pub signal_strength: u8,
    /// Статус регистрации
    pub status: NetworkStatus,
}

/// Статус вызова
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CallStatus {
    /// Нет активного вызова
    Idle,
    /// Исходящий вызов (набор номера)
    Dialing,
    /// Входящий вызов
    Ringing,
    /// Соединение установлено
    Active,
    /// Вызов завершен
    Disconnected,
    /// Ошибка вызова
    Error,
}

/// Информация о вызове
#[derive(Debug, Clone, PartialEq)]
pub struct CallInfo {
    /// Номер телефона
    pub number: String<20>,
    /// Статус вызова
    pub status: CallStatus,
    /// Длительность в секундах (для активных вызовов)
    pub duration: Option<u32>,
}

/// Информация о SIM-карте
#[derive(Debug, Clone, PartialEq)]
pub struct SimInfo {
    /// IMSI номер
    pub imsi: String<20>,
    /// Номер SIM-карты
    pub sim_number: String<20>,
    /// Статус PIN
    pub pin_required: bool,
}

/// Уровни громкости (0-9)
pub type VolumeLevel = u8;

/// DTMF тоны для набора номера
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DtmfTone {
    /// Цифра 0
    Digit0,
    /// Цифра 1
    Digit1,
    /// Цифра 2
    Digit2,
    /// Цифра 3
    Digit3,
    /// Цифра 4
    Digit4,
    /// Цифра 5
    Digit5,
    /// Цифра 6
    Digit6,
    /// Цифра 7
    Digit7,
    /// Цифра 8
    Digit8,
    /// Цифра 9
    Digit9,
    /// Символ звездочки (*)
    Star,
    /// Символ решетки (#)
    Hash,
    /// DTMF тон A
    A,
    /// DTMF тон B
    B,
    /// DTMF тон C
    C,
    /// DTMF тон D
    D,
}

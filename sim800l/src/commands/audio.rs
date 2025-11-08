//! Команды для работы с аудио

/// Управление громкостью
/// Установить громкость динамика (0-9)
pub const AT_SPEAKER_VOLUME: &str = "AT+CLVL=";     // Громкость динамика (0-9)
/// Установить усиление микрофона (0-15)
pub const AT_MICROPHONE_GAIN: &str = "AT+CMIC=";    // Усиление микрофона (0-15)

/// Аудио пути
/// Переключить на трубку
pub const AT_AUDIO_PATH_HANDSET: &str = "AT+CHFA=0"; // Трубка
/// Переключить на гарнитуру
pub const AT_AUDIO_PATH_HEADSET: &str = "AT+CHFA=1"; // Гарнитура
/// Переключить на громкую связь
pub const AT_AUDIO_PATH_SPEAKER: &str = "AT+CHFA=2"; // Громкая связь

/// TTS (Text-to-Speech)
/// Воспроизвести текст голосом
pub const AT_TTS_PLAY: &str = "AT+CTTS=";           // Воспроизвести текст
/// Остановить воспроизведение TTS
pub const AT_TTS_STOP: &str = "AT+CTTS=0";          // Остановить TTS
/// Установить громкость TTS
pub const AT_TTS_VOLUME: &str = "AT+CTTSV=";        // Громкость TTS

/// Управление звонком
/// Установить громкость звонка
pub const AT_RING_VOLUME: &str = "AT+CRSL=";        // Громкость звонка
/// Установить мелодию звонка
pub const AT_RING_TONE: &str = "AT+CRTONE=";        // Мелодия звонка

/// Аудио запись/воспроизведение
/// Начать запись аудио
pub const AT_AUDIO_RECORD: &str = "AT+AREC=";       // Запись аудио
/// Воспроизвести аудио файл
pub const AT_AUDIO_PLAY: &str = "AT+APLAY=";        // Воспроизведение
/// Остановить аудио операцию
pub const AT_AUDIO_STOP: &str = "AT+ASTOP";         // Остановить аудио

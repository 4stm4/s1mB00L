//! Абстракция для работы с таймерами

/// Трейт для работы с системным временем
pub trait Timer {
    /// Получить текущее время в миллисекундах
    fn now_ms(&self) -> u32;
    
    /// Ждать указанное количество миллисекунд
    fn delay_ms(&mut self, ms: u32);
}

/// Заглушка таймера для тестирования
pub struct MockTimer {
    current_time: u32,
}

impl MockTimer {
    /// Создает новый экземпляр mock таймера
    pub fn new() -> Self {
        Self { current_time: 0 }
    }
    
    /// Принудительно продвигает время вперед (для тестирования)
    pub fn advance(&mut self, ms: u32) {
        self.current_time += ms;
    }
}

impl Timer for MockTimer {
    fn now_ms(&self) -> u32 {
        self.current_time
    }
    
    fn delay_ms(&mut self, ms: u32) {
        self.current_time += ms;
        // В моке не делаем реальную задержку
    }
}

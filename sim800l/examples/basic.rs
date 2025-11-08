//! Базовый пример использования библиотеки SIM800L

use std::thread;
use std::time::Duration;

use sim800l::{Sim800L, Timer, NetworkStatus};

fn main() {
    println!("Запуск примера SIM800L...");
    
    // Для примера используем заглушки
    let uart = MockUart::new();
    let timer = MockTimer::new();
    
    // Создаем экземпляр драйвера SIM800L с таймером
    let mut modem = Sim800L::new(uart, timer);
    
    // Инициализация модема
    match modem.init() {
        Ok(()) => {
            println!("Модем успешно инициализирован");
        }
        Err(_e) => {
            println!("Ошибка инициализации модема");
            return;
        }
    }
    
    // Получаем версию прошивки
    match modem.get_firmware_version() {
        Ok(_version) => {
            println!("Версия получена успешно");
        }
        Err(_) => {
            println!("Ошибка получения версии");
        }
    }
    
    // Демонстрация базовых функций
    for i in 0..3 {
        println!("Итерация {}", i + 1);
        
        // Проверяем статус сети
        match modem.check_network_status() {
            Ok(NetworkStatus::RegisteredHome) | Ok(NetworkStatus::RegisteredRoaming) => {
                println!("Сеть доступна");
                
                // Пытаемся отправить SMS
                match modem.send_sms("+1234567890", "Hello from SIM800L!") {
                    Ok(sms_response) => {
                        if sms_response.sent {
                            println!("SMS отправлено успешно");
                            if let Some(id) = sms_response.message_id {
                                println!("ID сообщения: {}", id);
                            }
                        }
                    }
                    Err(_) => {
                        println!("Ошибка отправки SMS");
                    }
                }
            }
            Ok(_) => {
                println!("Сеть недоступна");
            }
            Err(_) => {
                println!("Ошибка проверки статуса сети");
            }
        }
        
        // Пауза между итерациями
        thread::sleep(Duration::from_secs(2));
    }
    
    println!("Пример завершен");
}

// Заглушки для демонстрации
struct MockUart;

impl MockUart {
    fn new() -> Self {
        Self
    }
}

impl embedded_hal::serial::Write<u8> for MockUart {
    type Error = ();
    
    fn write(&mut self, _word: u8) -> nb::Result<(), Self::Error> {
        Ok(())
    }
    
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}

impl embedded_hal::serial::Read<u8> for MockUart {
    type Error = ();
    
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        Err(nb::Error::WouldBlock)
    }
}

// Простая заглушка таймера для примера
struct MockTimer {
    counter: u32,
}

impl MockTimer {
    fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Timer for MockTimer {
    fn now_ms(&self) -> u32 {
        self.counter
    }
    
    fn delay_ms(&mut self, ms: u32) {
        self.counter += ms;
        // В реальном коде здесь должна быть настоящая задержка
    }
}

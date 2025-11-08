//! Простой пример демонстрации асинхронного API
//! 
//! Этот пример показывает разницу между синхронным и асинхронным использованием
//! без внешних зависимостей

#![no_std]

use sim800l::{MockTimer, Result};

// Пример синхронного использования
fn sync_example() -> Result<()> {
    // Создание мок-компонентов для демонстрации
    let _timer = MockTimer::new();
    
    // В реальном коде:
    // let uart = MockUart::new();
    // let mut modem = Sim800L::new(uart, timer);
    // modem.init()?;
    // let _sms_response = modem.send_sms("+1234567890", "Sync Hello!")?;
    // let _network_status = modem.check_network_status()?;
    
    Ok(())
}

// Демонстрация асинхронного API (концептуальный пример)
async fn async_example() -> Result<()> {
    // В реальном коде MockUart будет экспортирован
    let _timer = MockTimer::new();
    
    // В реальном коде:
    // let uart = MockUart::new();
    // let mut modem = AsyncSim800L::new(uart, timer);
    // modem.init().await?;
    // let _sms_response = modem.send_sms_async("+1234567890", "Async Hello!").await?;
    
    Ok(())
}

fn main() {
    // Демонстрация синхронного API
    let _ = sync_example();
    
    // Асинхронный API требует async runtime
    // В реальном embedded коде используйте Embassy, RTIC и т.д.
    
    loop {
        // Основной цикл программы
    }
}

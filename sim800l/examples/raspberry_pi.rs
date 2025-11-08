//! Пример использования SIM800L на Raspberry Pi Zero 2W
//! 
//! Этот пример показывает как использовать библиотеку в Linux окружении
//! с стандартным serial API

// Включаем std для Raspberry Pi
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use sim800l::{Sim800L, LinuxUart, LinuxTimer};

    println!("🍓 Запуск SIM800L на Raspberry Pi Zero 2W");

    // Подключение к UART (обычно /dev/ttyS0 или /dev/ttyAMA0)
    let uart = LinuxUart::new("/dev/ttyS0")?;
    let timer = LinuxTimer::new();

    // Создание драйвера
    let mut modem = Sim800L::new(uart, timer);

    println!("📡 Инициализация SIM800L...");
    modem.init()?;

    println!("✅ SIM800L готов к работе!");

    // Проверка сети
    println!("🔍 Проверка сети...");
    let network_status = modem.check_network_status()?;
    println!("📶 Статус сети: {:?}", network_status);

    // Получение IMEI
    println!("🆔 Получение IMEI...");
    let imei = modem.get_imei()?;
    println!("📱 IMEI: {}", imei);

    // Отправка SMS
    println!("📤 Отправка SMS...");
    let sms_response = modem.send_sms("+1234567890", "Hello from Raspberry Pi Zero 2W!")?;
    println!("✅ SMS отправлен: {:?}", sms_response);

    // Проверка батареи модема
    println!("🔋 Проверка батареи...");
    let battery = modem.get_battery_status()?;
    println!("🔋 Батарея: {:?}", battery);

    println!("🎉 Все операции выполнены успешно!");

    Ok(())
}

#[cfg(all(feature = "linux", feature = "tokio"))]
#[tokio::main]
async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    use sim800l::{AsyncSim800L, LinuxUart, LinuxTimer};

    println!("🍓 Асинхронный SIM800L на Raspberry Pi Zero 2W");

    let uart = LinuxUart::new("/dev/ttyS0")?;
    let timer = LinuxTimer::new();

    let mut modem = AsyncSim800L::new(uart, timer);

    println!("📡 Асинхронная инициализация...");
    modem.init().await?;

    println!("🚀 Запуск параллельных операций...");
    
    // Выполняем несколько операций параллельно - БЫСТРО!
    let (network_result, imei_result, battery_result) = tokio::join!(
        modem.check_network_status_async(),
        modem.get_imei_async(),
        modem.get_battery_status_async()
    );

    println!("📶 Сеть: {:?}", network_result?);
    println!("📱 IMEI: {:?}", imei_result?);
    println!("🔋 Батарея: {:?}", battery_result?);

    // Отправка SMS асинхронно
    println!("📤 Асинхронная отправка SMS...");
    let sms_response = modem.send_sms_async("+1234567890", "Async Pi Zero 2W!").await?;
    println!("✅ SMS отправлен: {:?}", sms_response);

    println!("🎉 Все асинхронные операции выполнены!");

    Ok(())
}

#[cfg(not(feature = "linux"))]
fn main() {
    println!("❌ Этот пример требует включения feature 'linux'");
    println!("Запустите: cargo run --example raspberry_pi --features=linux");
}

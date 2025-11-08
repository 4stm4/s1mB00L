//! Пример использования SIM800L с Embassy async runtime
//! 
//! Этот пример показывает как использовать асинхронный драйвер SIM800L
//! с Embassy framework для неблокирующих операций.

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_stm32::{
    bind_interrupts,
    peripherals,
    usart::{self, Config, Uart},
};
use sim800l::{AsyncSim800L, Error};

// Привязываем прерывания для UART1
bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

/// Реализация Timer trait для Embassy
struct EmbassyTimer;

impl sim800l::Timer for EmbassyTimer {
    fn delay_ms(&mut self, ms: u32) {
        // Embassy не поддерживает блокирующий delay в async контексте
        // Это только для примера - в реальном коде используйте Timer::after()
    }
    
    fn now_ms(&self) -> u32 {
        // Получаем текущее время в миллисекундах
        embassy_time::Instant::now().as_millis() as u32
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    
    // Настройка UART для SIM800L
    let mut config = Config::default();
    config.baudrate = 115200;
    
    let uart = Uart::new(
        p.USART1,
        p.PA10, // RX
        p.PA9,  // TX
        Irqs,
        p.DMA1_CH4, // TX DMA
        p.DMA1_CH5, // RX DMA
        config,
    ).unwrap();
    
    // Создание асинхронного драйвера
    let mut modem = AsyncSim800L::new(uart, EmbassyTimer);
    
    // Запуск основных задач
    spawner.spawn(sms_task(&mut modem)).unwrap();
    spawner.spawn(heartbeat_task()).unwrap();
    
    // Инициализация модема
    defmt::info!("Инициализация SIM800L...");
    match modem.init().await {
        Ok(()) => defmt::info!("SIM800L готов к работе"),
        Err(e) => {
            defmt::error!("Ошибка инициализации SIM800L: {:?}", e);
            return;
        }
    }
    
    // Основной цикл
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

/// Задача для отправки SMS каждые 30 секунд
#[embassy_executor::task]
async fn sms_task(modem: &'static mut AsyncSim800L<impl embedded_hal::serial::Read<u8> + embedded_hal::serial::Write<u8>, EmbassyTimer>) {
    loop {
        Timer::after(Duration::from_secs(30)).await;
        
        defmt::info!("Отправка SMS...");
        match modem.send_sms_async("+1234567890", "Статус: OK").await {
            Ok(response) => {
                defmt::info!("SMS отправлен: {:?}", response);
            }
            Err(Error::Timeout) => {
                defmt::warn!("Таймаут при отправке SMS");
            }
            Err(e) => {
                defmt::error!("Ошибка отправки SMS: {:?}", e);
            }
        }
    }
}

/// Задача индикации работы системы
#[embassy_executor::task]
async fn heartbeat_task() {
    loop {
        defmt::info!("💚 Система работает");
        Timer::after(Duration::from_secs(10)).await;
    }
}

#![no_std]
#![no_main]

use chrono::{NaiveDate, NaiveDateTime};
use cortex_m::interrupt::CriticalSection;
use defmt::*;
// use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_executor::Spawner;
use embassy_stm32::adc;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::low_level::Pin;
use embassy_stm32::rtc::{Rtc, RtcConfig};

// use embassy_stm32::spi::Spi;
// use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::Config;
use embassy_time::{Delay, Timer}; //Duration,

// use embedded_sdmmc::{
//     Block, BlockCount, BlockDevice, BlockIdx, Controller, Error, Mode, TimeSource, Timestamp, VolumeIdx,
// };
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // let mut config = Config::default();
    // {
    //     use embassy_stm32::rcc::*;
    //     config.rcc.ls = LsConfig::default_lsi();
    //     // config.rcc.hse = Some(Hse {
    //     //     freq: Hertz(32_000_000),
    //     //     mode: HseMode::Bypass,
    //     //     prescaler: HsePrescaler::DIV1,
    //     // });
    //     config.rcc.msi = Some(MSIRange::RANGE16M);
    //     config.rcc.mux = ClockSrc::PLL1_R;
    //     config.rcc.pll = Some(Pll {
    //         source: PllSource::MSI,
    //         prediv: PllPreDiv::DIV1,
    //         mul: PllMul::MUL8,
    //         divp: Some(PllPDiv::DIV2),
    //         divq: Some(PllQDiv::DIV2), // 64Mhz PLL1_Q clock (16 / 1 * 8 / 2), used for RNG
    //         divr: Some(PllRDiv::DIV4), // sysclk 32Mhz clock (16 / 1 * 8 / 4)
    //     });
    // }
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.ls = LsConfig::default_lsi();
        // config.rcc.hse = Some(Hse {
        //     freq: Hertz(32_000_000),
        //     mode: HseMode::Bypass,
        //     prescaler: HsePrescaler::DIV1,
        // });
        config.rcc.msi = Some(MSIRange::RANGE200K);
        config.rcc.mux = ClockSrc::PLL1_R;
        // config.rcc.mux = ClockSrc::;
        config.rcc.pll = Some(Pll {
            source: PllSource::MSI,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL8,
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV2), // 64Mhz PLL1_Q clock (16 / 1 * 8 / 2), used for RNG
            divr: Some(PllRDiv::DIV4), // sysclk 32Mhz clock (16 / 1 * 8 / 4)
        });
    }
    let p = embassy_stm32::init(config);
    info!("Hello World!");

    let now = NaiveDate::from_ymd_opt(2020, 5, 15)
        .unwrap()
        .and_hms_opt(10, 30, 15)
        .unwrap();

    let mut rtc = Rtc::new(p.RTC, RtcConfig::default());
    info!("Got RTC! {:?}", now.timestamp());

    rtc.set_datetime(now.into()).expect("datetime not set");

    // In reality the delay would be much longer
    Timer::after_millis(10000).await;
    // Timer::after_secs(20).await;

    let then: NaiveDateTime = rtc.now().unwrap().into();
    info!("Got RTC! {:?}", then.timestamp());
    // let duration: Duration = Duration.from_secs(12);
    let mut adc = embassy_stm32::adc::Adc::new(p.ADC, &mut Delay);
    let mut adc_pin = p.PB4;
    adc_pin.set_as_analog();

    adc.set_resolution(embassy_stm32::adc::Resolution::TwelveBit);
    adc.set_sample_time(embassy_stm32::adc::SampleTime::Cycles160_5);
    let mut temp_channel = adc.enable_temperature();

    let mut adc_value = adc.read(&mut temp_channel);
    info!("--> {}", adc_value);
    adc_value = adc.read(&mut adc_pin);
    info!("--> {}", adc_value);

    // rtc.start_wakeup_alarm(duration);
    // let spi_cfg = embassy_stm32::spi::Config::default();
    // Initialize SPI
    // let spi = Spi::new(
    //     p.SPI2, // peri
    //     p.PB13, // sck
    //     p.PA10, // mosi
    //     p.PB14, // miso
    //     NoDma,  // rxdma
    //     NoDma,  // txdma
    //     spi_cfg,
    // );
    //
    //     let write: [u8; 2] = [0x0A, 0x00];
    //     let mut read: [u8; 2] = [0u8; 2];
    //     unwrap!(spi.blocking_transfer_in_place(&mut read));
    //     info!("SPI read: {:?}", read);
    //
    //     let mut cont = embedded_sdmmc::Controller::new(embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs), time_source);
    //     write!(uart, "Init SD card...").unwrap();
    //     match cont.device().init() {
    //         Ok(_) => {
    //             write!(uart, "OK!\nCard size...").unwrap();
    //             match cont.device().card_size_bytes() {
    //                 Ok(size) => writeln!(uart, "{}", size).unwrap(),
    //                 Err(e) => writeln!(uart, "Err: {:?}", e).unwrap(),
    //             }
    //             write!(uart, "Volume 0...").unwrap();
    //             match cont.get_volume(embedded_sdmmc::VolumeIdx(0)) {
    //                 Ok(v) => writeln!(uart, "{:?}", v).unwrap(),
    //                 Err(e) => writeln!(uart, "Err: {:?}", e).unwrap(),
    //             }
    //         }
    //         Err(e) => writeln!(uart, "{:?}!", e).unwrap(),
    //     }
}

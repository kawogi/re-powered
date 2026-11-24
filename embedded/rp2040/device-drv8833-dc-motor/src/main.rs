#![expect(unused, clippy::unwrap_used, reason = "// TODO")]

//! Simple test program to drive the motor.
//!
//! TODO This is just a quick test and needs to be cleaned up.
#![no_std]
#![no_main]

mod status_led;

use embedded_hal::delay::DelayNs as _;
use embedded_hal::digital::OutputPin;
use embedded_hal::pwm::SetDutyCycle as _;
use panic_halt as _;
use smart_leds::RGB8;
use waveshare_rp2040_zero::entry;
use waveshare_rp2040_zero::hal::pwm::{InputHighRunning, Slices};
use waveshare_rp2040_zero::pac::Peripherals;
use waveshare_rp2040_zero::{
    Pins, XOSC_CRYSTAL_FREQ,
    hal::{
        Sio, clocks::init_clocks_and_plls, pac, pio::PIOExt as _, timer::Timer, watchdog::Watchdog,
    },
};

use crate::status_led::StatusLed;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then infinitely cycles the built-in LED colour from red, to green,
/// to blue and back to red.
#[entry]
fn main() -> ! {
    let peripherals =
        Peripherals::take().unwrap_or_else(|| unreachable!("The first call always succeeds"));

    let Peripherals {
        ADC: _,
        BUSCTRL: _,
        CLOCKS,
        DMA: _,
        I2C0: _,
        I2C1: _,
        IO_BANK0,
        IO_QSPI: _,
        PADS_BANK0,
        PADS_QSPI: _,
        PIO0,
        PIO1: _,
        PLL_SYS,
        PLL_USB,
        PPB: _,
        PSM: _,
        PWM,
        mut RESETS,
        ROSC: _,
        RTC: _,
        SIO,
        SPI0: _,
        SPI1: _,
        SYSCFG: _,
        SYSINFO: _,
        TBMAN: _,
        TIMER,
        UART0: _,
        UART1: _,
        USBCTRL_DPRAM: _,
        USBCTRL_REGS: _,
        VREG_AND_CHIP_RESET: _,
        WATCHDOG,
        XIP_CTRL: _,
        XIP_SSI: _,
        XOSC,
    } = peripherals;

    let mut watchdog = Watchdog::new(WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        XOSC,
        CLOCKS,
        PLL_SYS,
        PLL_USB,
        &mut RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let (mut pio, state_machine_0, _state_machine_1, _state_machine_2, _state_machine_3) =
        PIO0.split(&mut RESETS);

    let sio = Sio::new(SIO);
    let pins = Pins::new(IO_BANK0, PADS_BANK0, sio.gpio_bank0, &mut RESETS);
    let timer = Timer::new(TIMER, &mut RESETS, &clocks);
    let mut status_led = StatusLed::new(&mut pio, pins.neopixel, state_machine_0, &clocks, &timer);

    let control_tx = pins.gp0;
    let control_rx = pins.gp1;
    let motor_pwm_a = pins.gp2;
    let motor_pwm_b = pins.gp3;
    let mut motor_sleep = pins.gp29.into_push_pull_output();
    let motor_fault = pins.gp28;

    // Init PWMs
    let pwm_slices = Slices::new(PWM, &mut RESETS);

    // Configure PWM4
    let mut pwm = pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.enable();

    // Set to run when b channel is high
    //    let pwm = pwm.into_mode::<InputHighRunning>();

    let mut channel_a = pwm.channel_a;
    let channel_pin_a = channel_a.output_to(motor_pwm_a);
    let mut channel_b = pwm.channel_b;
    let channel_pin_b = channel_b.output_to(motor_pwm_b);

    // Set duty cycle
    let max_duty_cycle = channel_a.max_duty_cycle();
    channel_a.set_duty_cycle(max_duty_cycle / 5);
    //channel_a.set_inverted(); // Invert the output
    //channel_a.clr_inverted(); // Don't invert the output

    //motor_sleep.set_high();

    // Infinite colour wheel loop
    let mut n: u8 = 128;
    let mut mut_timer = timer; // rebind to force a copy of the timer
    loop {
        status_led.set_color(with_brightness(wheel(n), 32));
        n = n.wrapping_add(1);

        mut_timer.delay_ms(25);
    }
}

/// Convert a number from `0..=255` to an RGB color triplet.
///
/// The colors are a transition from red, to green, to blue and back to red.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        // No green in this sector - red and blue only
        (255 - (wheel_pos * 3), 0, wheel_pos * 3).into()
    } else if wheel_pos < 170 {
        // No red in this sector - green and blue only
        wheel_pos -= 85;
        (0, wheel_pos * 3, 255 - (wheel_pos * 3)).into()
    } else {
        // No blue in this sector - red and green only
        wheel_pos -= 170;
        (wheel_pos * 3, 255 - (wheel_pos * 3), 0).into()
    }
}

const fn with_brightness(color: RGB8, brightness: u8) -> RGB8 {
    RGB8 {
        r: (color.r as u16 * (brightness as u16 + 1) / 256) as u8,
        g: (color.g as u16 * (brightness as u16 + 1) / 256) as u8,
        b: (color.b as u16 * (brightness as u16 + 1) / 256) as u8,
    }
}

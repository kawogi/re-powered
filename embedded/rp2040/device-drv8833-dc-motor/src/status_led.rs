use core::iter::once;
use panic_halt as _;
use smart_leds::{RGB8, SmartLedsWrite as _};
use waveshare_rp2040_zero::hal::clocks::ClocksManager;
use waveshare_rp2040_zero::hal::gpio::bank0::Gpio16;
use waveshare_rp2040_zero::hal::gpio::{DefaultTypeState, Pin};
use waveshare_rp2040_zero::hal::gpio::{PullDown, ValidFunction};
use waveshare_rp2040_zero::hal::pio::StateMachineIndex;
use waveshare_rp2040_zero::hal::pio::{PIO, UninitStateMachine};
use waveshare_rp2040_zero::hal::timer::CountDown;
use waveshare_rp2040_zero::hal::{clocks::Clock as _, pio::PIOExt, timer::Timer};
use ws2812_pio::Ws2812;

/// The built-in RGB-LED on the board.
pub(crate) struct StatusLed<'timer, P: PIOExt, SM: StateMachineIndex> {
    led: Ws2812<P, SM, CountDown<'timer>, Pin<Gpio16, <P as PIOExt>::PinFunction, PullDown>>,
}

impl<'timer, P, SM> StatusLed<'timer, P, SM>
where
    P: PIOExt,
    SM: StateMachineIndex,
    Gpio16: ValidFunction<<P as PIOExt>::PinFunction>,
{
    pub(crate) fn new(
        pio: &mut PIO<P>,
        neopixel: Pin<
            Gpio16,
            <Gpio16 as DefaultTypeState>::Function,
            <Gpio16 as DefaultTypeState>::PullType,
        >,
        state_machine: UninitStateMachine<(P, SM)>,
        clocks: &ClocksManager,
        timer: &'timer Timer,
    ) -> Self {
        let led = Ws2812::new(
            neopixel.into_function(),
            pio,
            state_machine,
            clocks.peripheral_clock.freq(),
            timer.count_down(),
        );

        Self { led }
    }

    pub(crate) fn set_color(&mut self, color: impl Into<RGB8>) {
        self.led.write(once(color.into())).unwrap();
    }
}

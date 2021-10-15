#![no_std]
#![no_main]

use arduino_hal::hal;
use keyberon::action::Action;
use keyberon::debounce::Debouncer;
use keyberon::layout;
use keyberon::layout::Layout;
use keyberon::matrix::{Matrix, PressedKeys};
use panic_halt as _;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomActions {
    Reset,
}
const RESET: Action<CustomActions> = Action::Custom(CustomActions::Reset);

pub static LAYERS: keyberon::layout::Layers<CustomActions> = keyberon::layout::layout! {
    {[ {RESET} Q W E (1) ]}
    {[ A S D F G ]}
};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.led_tx.into_output();

    let d2 = pins.d2;
    let d3 = pins.d3;
    let d4 = pins.d4;
    let d5 = pins.d5;
    let d6 = pins.d6;

    let mut matrix: Matrix<
        hal::port::Pin<arduino_hal::port::mode::Input<arduino_hal::port::mode::PullUp>, _>,
        hal::port::Pin<arduino_hal::port::mode::Output>,
        4,
        1,
    > = avr_device::interrupt::free(move |_cs| {
        Matrix::new(
            [
                d2.into_pull_up_input().downgrade(),
                d3.into_pull_up_input().downgrade(),
                d4.into_pull_up_input().downgrade(),
                d5.into_pull_up_input().downgrade(),
            ],
            [d6.into_output().downgrade()],
        )
    })
    .unwrap();

    let mut layout = Layout::new(LAYERS);
    let mut debouncer: keyberon::debounce::Debouncer<keyberon::matrix::PressedKeys<4, 1>> =
        Debouncer::new(PressedKeys::default(), PressedKeys::default(), 10);
    // TODO: setup usb peripheral

    // TODO: use a hardware timer at 1kHz
    loop {
        for event in debouncer.events(matrix.get().unwrap()) {
            layout.event(event);
        }

        match layout.tick() {
            layout::CustomEvent::Press(event) => match event {
                CustomActions::Reset => led.toggle(),
            },
            _ => (),
        }
        // TODO: send usb report
        arduino_hal::delay_ms(1);
    }
}

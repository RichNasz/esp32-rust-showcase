// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Qualified BOOT press. A level that is already active at the start of the
//! wait is ignored, so a ROM download strap does not count as a press.

use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Input, InputConfig, Level, Pull};

use crate::board;

const DEBOUNCE: Duration = Duration::from_millis(40);
const POLL: Duration = Duration::from_millis(10);

pub async fn wait_for_new_press() {
    let button = Input::new(
        board::take_trigger(),
        InputConfig::default().with_pull(Pull::Up),
    );
    if is_pressed(&button) {
        while is_pressed(&button) {
            Timer::after(POLL).await;
        }
        Timer::after(DEBOUNCE).await;
    }
    loop {
        if is_pressed(&button) {
            Timer::after(DEBOUNCE).await;
            if is_pressed(&button) {
                return;
            }
        }
        Timer::after(POLL).await;
    }
}

fn is_pressed(button: &Input<'_>) -> bool {
    let active = if board::TRIGGER_ACTIVE_LOW {
        Level::Low
    } else {
        Level::High
    };
    button.level() == active
}

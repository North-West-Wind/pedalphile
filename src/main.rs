use std::{thread::sleep, time::Duration};

use config::load_config;
use mki::{Action, Event, Keyboard, State};
use module::RelativeKey;

mod config;
mod module;
mod state;

fn main() {
    load_config();
    println!("Running");
    mki::bind_any_key(double_fire_kb(|key, state| {
        use State::*;
        match state {
            Pressed => module::handle_key_press(RelativeKey::from_keyboard(key)),
            Released => module::handle_key_release(RelativeKey::from_keyboard(key)),
        }
    }));

    loop {
        sleep(Duration::from_secs(1));
    }
}

fn double_fire_kb(action: impl Fn(Keyboard, State) + Send + Sync + 'static) -> Action {
    Action {
        callback: Box::new(move |event, state| {
            if let Event::Keyboard(key) = event {
                action(key, state)
            }
        }),
        inhibit: mki::InhibitEvent::No,
        defer: true,
        sequencer: false,
    }
}
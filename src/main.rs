use std::{thread::sleep, time::Duration};

use config::load_config;
use mki::{Action, Event, Keyboard, State};
use state::setup;

mod config;
mod module;
mod state;

fn main() {
	setup();
    load_config();
    mki::bind_any_key(double_fire_kb(|keyboard, state| {
        use State::*;
        match state {
            Pressed => module::handle_key_press(&keyboard),
            Released => module::handle_key_release(&keyboard),
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
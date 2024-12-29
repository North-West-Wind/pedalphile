use std::{thread::sleep, time::Duration};

use mki::{Action, Keyboard};

mod module;
mod state;

fn main() {
    println!("PedalPhile is running");
    mki::bind_key(Keyboard::F13, Action::handle_kb(|_key| {
        module::handle_key(module::RelativeKey::Left);
    }));
    mki::bind_key(Keyboard::F14, Action::handle_kb(|_key| {
        module::handle_key(module::RelativeKey::Middle);
    }));
    mki::bind_key(Keyboard::F15, Action::handle_kb(|_key| {
        module::handle_key(module::RelativeKey::Right);
    }));

    loop {
        sleep(Duration::from_secs(1));
    }
}
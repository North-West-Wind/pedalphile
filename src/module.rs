use voice::VoiceModule;

use crate::state::{get_app, get_mut_app};

mod voice;

#[derive(PartialEq, Eq)]
pub enum RelativeKey {
	Left,
	Middle,
	Right,
	Invalid
}

pub fn handle_key(key: RelativeKey) {
	let app = get_mut_app();
	match key {
		RelativeKey::Middle => {
			toggle_cat_act();
		}
		RelativeKey::Left => {
			if app.module_change {
				input_cat_act(false);
			} else {
				app.module.handle_left();
			}
		}
		RelativeKey::Right => {
			if app.module_change {
				input_cat_act(true);
			} else {
				app.module.handle_right();
			}
		}
		_ => {}
	}
}

fn toggle_cat_act() {
	let app = get_mut_app();
	if app.module_change {
		app.module = Modules::get_module(app.module_tmp);
		app.module_tmp = 0;
		app.module_change = false;
		println!("Switched to module: {}", app.module.name());
	} else {
		app.module_change = true;
	}
}

fn input_cat_act(up: bool) {
	let app = get_mut_app();
	app.module_tmp <<= 1;
	if up {
		app.module_tmp += 1;
	}
}

pub trait LeftRightHandler {
	fn handle_left(&mut self);
	fn handle_right(&mut self);
}

pub enum Modules {
	Dummy,
	Voice(VoiceModule)
}

impl LeftRightHandler for Modules {
	fn handle_left(&mut self) {
		use Modules::*;
		match self {
			Voice(module) => module.handle_left(),
			_ => {}
		}
	}

	fn handle_right(&mut self) {
		use Modules::*;
		match self {
			Voice(module) => module.handle_right(),
			_ => {}
		}
	}
}

impl Modules {
	pub const fn get_module(cat: u8) -> Modules {
		match cat {
			0 => Modules::Voice(VoiceModule {  }),
			_ => Modules::Dummy
		}
	}

	pub fn name(&self) -> &str {
		match self {
			Modules::Dummy => "Dummy",
			Modules::Voice(_) => "Voice"
		}
	}
}
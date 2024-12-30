use soundboard::SoundboardModule;
use voice::VoiceModule;

use crate::state::get_mut_app;

mod soundboard;
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
			if app.module_change || !app.module.handle_middle() {
				toggle_cat_act();
			}
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
		println!("-> {}", app.module.name());
	} else {
		app.module_change = true;
	}
}

fn input_cat_act(up: bool) {
	let app = get_mut_app();
	app.module_tmp <<= 1;
	if up {
		app.module_tmp |= 1;
	}
}

pub trait LeftRightHandler {
	fn handle_left(&mut self);
	fn handle_right(&mut self);
}

pub trait MiddleHandler {
	// true = handled and no fallback
	fn handle_middle(&mut self) -> bool;
}

pub enum Modules {
	Dummy,
	Voice(VoiceModule),
	Soundboard(SoundboardModule),
}

impl LeftRightHandler for Modules {
	fn handle_left(&mut self) {
		use Modules::*;
		match self {
			Voice(module) => module.handle_left(),
			Soundboard(module) => module.handle_left(),
			_ => {}
		}
	}

	fn handle_right(&mut self) {
		use Modules::*;
		match self {
			Voice(module) => module.handle_right(),
			Soundboard(module) => module.handle_right(),
			_ => {}
		}
	}
}

impl MiddleHandler for Modules {
	fn handle_middle(&mut self) -> bool {
		use Modules::*;
		match self {
			Soundboard(module) => module.handle_middle(),
			_ => false
		}
	}
}

impl Modules {
	pub const fn get_module(cat: u8) -> Modules {
		use Modules::*;
		match cat {
			0 => Voice(VoiceModule {  }),
			1 => Soundboard(soundboard::create_module()),
			_ => Dummy
		}
	}

	pub fn name(&self) -> &str {
		use Modules::*;
		match self {
			Dummy => "Dummy",
			Voice(_) => "Voice",
			Soundboard(_) => "Soundboard"
		}
	}

	pub fn short_name(&self) -> &str {
		use Modules::*;
		match self {
			Dummy => "dum",
			Voice(_) => "voi",
			Soundboard(_) => "snd"
		}
	}
}
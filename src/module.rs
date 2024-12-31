use clicker::ClickerModule;
use mki::Keyboard;
use save::SaveModule;
use soundboard::SoundboardModule;
use voice::VoiceModule;

use crate::{config::save_config, state::{get_mut_app, SaveState}};

mod clicker;
mod save;
mod soundboard;
mod voice;

#[derive(PartialEq, Eq)]
pub enum RelativeKey {
	Left,
	Middle,
	Right,
	Invalid,
}

impl RelativeKey {
	pub fn keyboard(&self) -> Keyboard {
		use RelativeKey::*;
		match self {
			Left => Keyboard::F13,
			Middle => Keyboard::F14,
			Right => Keyboard::F15,
			Invalid => Keyboard::F24, // ignored
		}
	}

	pub fn from_keyboard(key: Keyboard) -> RelativeKey {
		use Keyboard::*;
		match key {
			F13 => RelativeKey::Left,
			F14 => RelativeKey::Middle,
			F15 => RelativeKey::Right,
			_ => RelativeKey::Invalid,
		}
	}
}

pub fn handle_key_press(key: RelativeKey) {
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
				app.module.handle_left_press();
			}
		}
		RelativeKey::Right => {
			if app.module_change {
				input_cat_act(true);
			} else {
				app.module.handle_right();
				app.module.handle_right_press();
			}
		},
		_ => {}
	}
}

pub fn handle_key_release(key: RelativeKey) {
	let app = get_mut_app();
	match key {
		RelativeKey::Left => {
			if !app.module_change {
				app.module.handle_left_release();
			}
		}
		RelativeKey::Right => {
			if !app.module_change {
				app.module.handle_right_release();
			}
		},
		_ => {}
	}
}

fn toggle_cat_act() {
	let app = get_mut_app();
	if app.module_change {
		app.module.save(&mut app.save_state);
		app.module = Modules::get_module(app.module_tmp);
		app.module.load(&app.save_state);
		app.save_state.module = app.module_tmp;
		app.module_tmp = 0;
		app.module_change = false;
		save_config();
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

pub trait LeftRightHoldHandler {
	fn handle_left_press(&mut self);
	fn handle_left_release(&mut self);
	fn handle_right_press(&mut self);
	fn handle_right_release(&mut self);
}

pub trait SaveStateUser {
	fn load(&mut self, save_state: &SaveState);
	fn save(&mut self, save_state: &mut SaveState);
}

pub enum Modules {
	Dummy,
	Voice(VoiceModule),
	Soundboard(SoundboardModule),
	Clicker(ClickerModule),

	Save(SaveModule),
}

impl LeftRightHandler for Modules {
	fn handle_left(&mut self) {
		use Modules::*;
		match self {
			Voice(module) => module.handle_left(),
			Soundboard(module) => module.handle_left(),
			Save(module) => module.handle_left(),
			_ => {}
		}
	}

	fn handle_right(&mut self) {
		use Modules::*;
		match self {
			Voice(module) => module.handle_right(),
			Soundboard(module) => module.handle_right(),
			Save(module) => module.handle_right(),
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

impl LeftRightHoldHandler for Modules {
	fn handle_left_press(&mut self) {
		use Modules::*;
		match self {
			Clicker(module) => module.handle_left_press(),
			_ => {}
		}
	}

	fn handle_left_release(&mut self) {
		use Modules::*;
		match self {
			Clicker(module) => module.handle_left_release(),
			_ => {}
		}
	}

	fn handle_right_press(&mut self) {
		use Modules::*;
		match self {
			Clicker(module) => module.handle_right_press(),
			_ => {}
		}
	}

	fn handle_right_release(&mut self) {
		use Modules::*;
		match self {
			Clicker(module) => module.handle_right_release(),
			_ => {}
		}
	}
}

impl SaveStateUser for Modules {
	fn load(&mut self, save_state: &SaveState) {
		use Modules::*;
		match self {
			Soundboard(module) => module.load(save_state),
			_ => {}
		}
	}

	fn save(&mut self, save_state: &mut SaveState) {
		use Modules::*;
		match self {
			Soundboard(module) => module.save(save_state),
			_ => {}
		}
	}
}

impl Modules {
	pub const fn get_module(cat: u8) -> Modules {
		use Modules::*;
		match cat {
			0 => Voice(VoiceModule {  }),
			1 => Soundboard(soundboard::create_module()),
			2 => Clicker(clicker::create_module()),
			255 => Save(SaveModule {  }),
			_ => Dummy
		}
	}

	pub fn name(&self) -> &str {
		use Modules::*;
		match self {
			Dummy => "Dummy",
			Voice(_) => "Voice",
			Soundboard(_) => "Soundboard",
			Clicker(_) => "Clicker",
			Save(_) => "Save",
		}
	}

	pub fn short_name(&self) -> &str {
		use Modules::*;
		match self {
			Dummy => "dum",
			Voice(_) => "voi",
			Soundboard(_) => "snd",
			Clicker(_) => "clk",
			Save(_) => "sav",
		}
	}
}
use mki::Keyboard;

use crate::{config::save_config, state::get_mut_app};

use super::LeftRightHandler;

pub struct SaveModule {}

impl LeftRightHandler for SaveModule {
	fn handle_left(&mut self) {
		if Keyboard::F15.is_pressed() {
			save();
		}
	}

	fn handle_right(&mut self) {
		if Keyboard::F13.is_pressed() {
			save();
		}
	}
}

fn save() {
	save_config();
	get_mut_app().println("Saved states".to_string());
}
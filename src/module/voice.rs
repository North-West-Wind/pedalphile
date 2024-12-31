use std::process::Command;

use super::{LeftRightHandler, RelativeKey};

pub struct VoiceModule {}

impl LeftRightHandler for VoiceModule {
	fn handle_left(&mut self) {
		if RelativeKey::Right.keyboard().is_pressed() {
			let _ = Command::new("pactl").args([
				"set-source-mute",
				"in_game",
				"toggle"
			]).output();
		}
	}

	fn handle_right(&mut self) {
		if  RelativeKey::Left.keyboard().is_pressed() {
			let _ = Command::new("pactl").args([
				"set-source-mute",
				"in_vc",
				"toggle"
			]).output();
		}
	}
}
use std::{process::Command, time::Instant};

use super::{LeftRightHandler, RelativeKey};

pub(super) struct VoiceModule {}

impl LeftRightHandler for VoiceModule {
	fn handle_left(&mut self) {
		if mki::Keyboard::F15.is_pressed() {
			let _ = Command::new("pactl").args([
				"set-source-mute",
				"in_game",
				"toggle"
			]).output();
		}
	}

	fn handle_right(&mut self) {
		if mki::Keyboard::F13.is_pressed() {
			let _ = Command::new("pactl").args([
				"set-source-mute",
				"in_vc",
				"toggle"
			]).output();
		}
	}
}
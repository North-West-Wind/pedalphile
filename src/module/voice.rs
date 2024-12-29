use std::process::Command;

use super::LeftRightHandler;

pub(super) struct VoiceModule { }

impl LeftRightHandler for VoiceModule {
	fn handle_left(&mut self) {
		let _ = Command::new("pactl").args([
			"set-source-mute",
			"in_game",
			"toggle"
		]).output();
	}

	fn handle_right(&mut self) {
		let _ = Command::new("pactl").args([
			"set-source-mute",
			"in_vc",
			"toggle"
		]).output();
	}
}
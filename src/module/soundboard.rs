use std::process::Command;

use crate::state::{get_mut_app, SaveState};

use super::{LeftRightHandler, MiddleHandler, RelativeKey, SaveStateUser};

#[derive(PartialEq, Eq)]
enum EditMode {
	None,
	Dir,
	File,
}

pub struct SoundboardModule {
	edit_mode: EditMode,
	id: u32,
	dir: u8,
	file: u8,
	tmp: u8,
	val_changed: bool,
}

pub(super) const fn create_module() -> SoundboardModule {
	SoundboardModule {
		edit_mode: EditMode::None,
		id: 0,
		dir: 0,
		file: 0,
		tmp: 0,
		val_changed: false,
	}
}

impl LeftRightHandler for SoundboardModule {
	fn handle_left(&mut self) {
		if self.edit_mode != EditMode::None {
			self.tmp <<= 1;
			self.val_changed = true;
		}
	}

	fn handle_right(&mut self) {
		if self.edit_mode != EditMode::None {
			self.tmp = (self.tmp << 1) | 1;
			self.val_changed = true;
		} else if  RelativeKey::Left.keyboard().is_pressed() {
			let _ = Command::new("cls").args([
				"stop"
			]).output();
		} else {
			let _ = Command::new("cls").args([
				"play-id",
				self.id.to_string().as_str()
			]).output();
		}
	}
}

impl MiddleHandler for SoundboardModule {
	fn handle_middle(&mut self) -> bool {
		if !RelativeKey::Left.keyboard().is_pressed() && self.edit_mode == EditMode::None {
			return false;
		}

		if self.edit_mode == EditMode::None {
			self.edit_mode = EditMode::Dir;
		} else if self.edit_mode == EditMode::Dir {
			self.edit_mode = EditMode::File;
			if self.val_changed {
				self.dir = self.tmp;
				get_mut_app().println(format!("{}/", self.dir));
			}
		} else if self.edit_mode == EditMode::File {
			self.edit_mode = EditMode::None;
			if self.val_changed {
				self.file = self.tmp;
			}
			self.id = ((self.dir as u32) << 5) | self.file as u32;
			get_mut_app().println(format!("{}/{}", self.dir, self.file));
		}
		self.tmp = 0;

		return true;
	}
}

impl SaveStateUser for SoundboardModule {
	fn load(&mut self, save_state: &SaveState) {
		self.id = save_state.soundboard_id;
		self.dir = (self.id >> 5) as u8;
		self.file = (self.id & 0x1F) as u8;
	}

	fn save(&mut self, save_state: &mut SaveState) {
		save_state.soundboard_id = self.id;
	}
}
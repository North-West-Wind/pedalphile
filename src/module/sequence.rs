use std::{collections::HashMap, str::FromStr};

use mki::Keyboard;

use crate::state::{get_mut_app, SaveState};

use super::{LeftRightHandler, MiddleHandler, OtherKeyHandler, RelativeKey, SaveStateUser};

#[derive(PartialEq, Eq)]
enum EditMode {
	None,
	Select,
	Record
}

pub struct SequenceModule {
	edit_mode: EditMode,
	id: u32,
	tmp: u32,
	sequences: HashMap<u32, Vec<Keyboard>>
}

pub fn create_module() -> SequenceModule {
	SequenceModule {
		edit_mode: EditMode::None,
		id: 0,
		tmp: 0,
		sequences: HashMap::new()
	}
}

impl LeftRightHandler for SequenceModule {
	fn handle_left(&mut self) {
		if self.edit_mode != EditMode::None {
			self.tmp <<= 1;
		}
	}

	fn handle_right(&mut self) {
		if self.edit_mode != EditMode::None {
			self.tmp = (self.tmp << 1) | 1;
		} else if RelativeKey::Left.keyboard().is_pressed() {
			self.edit_mode = EditMode::Record;
		} else {
			let sequence = self.sequences.get(&self.id);
			if sequence.is_some() {
				let sequence = sequence.unwrap();
				for key in sequence {
					key.press();
				}
				for key in sequence {
					key.release();
				}
			}
		}
	}
}

impl MiddleHandler for SequenceModule {
	fn handle_middle(&mut self) -> bool {
		if !RelativeKey::Left.keyboard().is_pressed() && self.edit_mode == EditMode::None {
			return false;
		}

		if self.edit_mode == EditMode::None {
			self.edit_mode = EditMode::Select;
		} else if self.edit_mode == EditMode::Select {
			self.edit_mode = EditMode::None;
			if self.id != self.tmp {
				self.id = self.tmp;
				get_mut_app().println(format!("{}", self.id));
			}
		} else if self.edit_mode == EditMode::Record {
			if RelativeKey::Left.keyboard().is_pressed() {
				self.sequences.remove(&self.id);
				get_mut_app().println(format!("rm {}", self.id));
			}
			self.edit_mode = EditMode::None;
		}
		self.tmp = 0;

		return true;
	}
}

impl OtherKeyHandler for SequenceModule {
	fn handle_key_press(&mut self, key: &Keyboard) {
		if self.edit_mode != EditMode::Record {
			return;
		}

		if !self.sequences.contains_key(&self.id) {
			self.sequences.insert(self.id, vec![]);
		}

		let sequence = self.sequences.get_mut(&self.id).unwrap();
		sequence.push(*key);
		get_mut_app().println(format!("+ {:?}", key));
	}

	fn handle_key_release(&mut self, _: &Keyboard) {
		// ignored
	}
}

impl SaveStateUser for SequenceModule {
	fn load(&mut self, save_state: &SaveState) {
		for (id, keys)in save_state.sequences.clone().into_iter() {
			let mut keyboards = vec![];
			for key in keys {
				let result = Keyboard::from_str(&key);
				if result.is_ok() {
					keyboards.push(result.unwrap());
				} else {
					let parsed = key.parse::<i32>();
					if parsed.is_err() {
						keyboards.clear();
						break;
					} else {
						keyboards.push(Keyboard::Other(parsed.unwrap()));
					}
				}
			}
			if !keyboards.is_empty() {
				self.sequences.insert(id, keyboards);
			}
		}
	}

	fn save(&mut self, save_state: &mut SaveState) {
		save_state.sequences.clear();
		for (id, keyboards) in self.sequences.clone().into_iter() {
			let mut keys = vec![];
			for keyboard in keyboards {
				match keyboard {
					Keyboard::Other(code) => keys.push(code.to_string()),
					_ => keys.push(format!("{:?}", keyboard)),
				}
			}
			save_state.sequences.insert(id, keys);
		}
	}
}
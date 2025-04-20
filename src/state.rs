use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::module::Modules;

static mut APP: Option<App> = Option::None;
pub struct App {
	pub module_change: bool,
	pub module_tmp: u8,
	pub module: Modules,
	pub save_state: SaveState,
}

pub fn create_app() -> App {
	App {
		module_change: false,
		module_tmp: 0,
		module: Modules::Dummy,
		save_state: create_save_state(),
	}
}

pub fn get_mut_app() -> &'static mut App {
	unsafe { APP.as_mut().unwrap() }
}

pub fn setup() {
	unsafe { APP = Option::Some(create_app()) };
}

impl App {
	pub fn println(&self, str: String) {
		println!("{}) {}", self.module.short_name(), str);
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveState {
	pub module: u8,
	pub soundboard_id: u32,
	pub sequences: HashMap<u32, Vec<String>>
}

fn create_save_state() -> SaveState {
	SaveState {
		module: 0,
		soundboard_id: 0,
		sequences: HashMap::new()
	}
}
use std::ptr::addr_of_mut;

use serde::{Deserialize, Serialize};

use crate::module::Modules;

static mut APP: App = create_app();
pub struct App {
	pub module_change: bool,
	pub module_tmp: u8,
	pub module: Modules,
	pub save_state: SaveState,
}

const fn create_app() -> App {
	App {
		module_change: false,
		module_tmp: 0,
		module: Modules::get_module(0),
		save_state: create_save_state(),
	}
}

pub fn get_mut_app() -> &'static mut App {
	unsafe { &mut *(addr_of_mut!(APP)) }
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
}

const fn create_save_state() -> SaveState {
	SaveState {
		module: 0,
		soundboard_id: 0,
	}
}
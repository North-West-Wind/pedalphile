use std::ptr::{addr_of, addr_of_mut};

use crate::module::Modules;

static mut APP: App = create_app();
pub struct App {
	pub module_change: bool,
	pub module_tmp: u8,
	pub module: Modules,
}

const fn create_app() -> App {
	return App {
		module_change: false,
		module_tmp: 0,
		module: Modules::get_module(0)
	}
}

pub fn get_mut_app() -> &'static mut App {
	unsafe { &mut *(addr_of_mut!(APP)) }
}

pub fn get_app() -> &'static App {
	unsafe { &*(addr_of!(APP)) }
}
use crate::state::get_mut_app;

use super::{LeftRightHoldHandler, MiddleHandler, RelativeKey};

pub struct ClickerModule {
	init: bool,
	left_clicking: bool,
	right_clicking: bool,
}

pub const fn create_module() -> ClickerModule {
	ClickerModule {
		init: false,
		left_clicking: false,
		right_clicking: false,
	}
}

impl LeftRightHoldHandler for ClickerModule {
	fn handle_left_press(&mut self) {
		if !RelativeKey::Right.keyboard().is_pressed() {
			return;
		}
		if !self.init {
			// KDE asks for permission before allowing clicks
			// It freezes the program, making it unable to detect release
			// Do a single press first before hold clicks
			get_mut_app().println("Init first!".to_string());
			return;
		}
		self.left_clicking = true;
		// by the time this gets called, it should be in a thread
		while self.left_clicking {
			mki::Mouse::Left.click();
			std::thread::sleep(std::time::Duration::from_millis(10));
		}
	}

	fn handle_left_release(&mut self) {
		self.left_clicking = false;
	}

	fn handle_right_press(&mut self) {
		if !RelativeKey::Left.keyboard().is_pressed() {
			return;
		}
		if !self.init {
			get_mut_app().println("Init first!".to_string());
			return;
		}
		self.right_clicking = true;
		// by the time this gets called, it should be in a thread
		while self.right_clicking {
			mki::Mouse::Right.click();
			std::thread::sleep(std::time::Duration::from_millis(10));
		}
	}

	fn handle_right_release(&mut self) {
		self.right_clicking = false;
	}
}

impl MiddleHandler for ClickerModule {
	fn handle_middle(&mut self) -> bool {
		if RelativeKey::Left.keyboard().is_pressed() {
			mki::Mouse::Left.click();
			get_mut_app().println("L".to_string());
			self.init = true;
			return true;
		}
		if RelativeKey::Right.keyboard().is_pressed() {
			mki::Mouse::Right.click();
			get_mut_app().println("R".to_string());
			self.init = true;
			return true;
		}
		return false;
	}
}
use super::{LeftRightHoldHandler, RelativeKey};

pub struct ClickerModule {
	left_clicking: bool,
	right_clicking: bool,
}

pub const fn create_module() -> ClickerModule {
	ClickerModule {
		left_clicking: false,
		right_clicking: false,
	}
}

impl LeftRightHoldHandler for ClickerModule {
	fn handle_left_press(&mut self) {
		if !RelativeKey::Right.keyboard().is_pressed() {
			return;
		}
		self.left_clicking = true;
		// by the time this gets called, it should be in a thread
		while self.left_clicking {
			mki::Mouse::Left.click();
			std::thread::sleep(std::time::Duration::from_millis(5));
		}
	}

	fn handle_left_release(&mut self) {
		self.left_clicking = false;
	}

	fn handle_right_press(&mut self) {
		if !RelativeKey::Left.keyboard().is_pressed() {
			return;
		}
		self.right_clicking = true;
		// by the time this gets called, it should be in a thread
		while self.right_clicking {
			mki::Mouse::Right.click();
			std::thread::sleep(std::time::Duration::from_millis(5));
		}
	}

	fn handle_right_release(&mut self) {
		self.right_clicking = false;
	}
}
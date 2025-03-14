use std::{io::Write, path::PathBuf};

use config::Config;

use crate::{module::{Modules, SaveStateUser}, state::{get_mut_app, SaveState}};

const APP_NAME: &str = "pedalphile";

fn get_config_path() -> PathBuf {
	dirs::config_dir().unwrap().join(APP_NAME).join("savestate.json")
}

fn build_config() -> Config {
	let save_state = &get_mut_app().save_state;
	Config::builder()
		.add_source(config::File::new(get_config_path().to_str().unwrap(), config::FileFormat::Json))
		// set default of save state
		.set_default("module", save_state.module).unwrap()
		.set_default("soundboard_id", save_state.soundboard_id).unwrap()
		.build()
		.unwrap()
}

pub(super) fn load_config() {
	let path = get_config_path();
	if !path.exists() {
		let _ = std::fs::create_dir_all(path.parent().unwrap());
		let output = std::fs::File::create(path.to_str().unwrap());
		let _ = output.unwrap().write_all(b"{}");
	}

	let settings = build_config();

	let parsed = settings.try_deserialize::<SaveState>();
	if parsed.is_err() {
		panic!("{:?}", parsed.unwrap_err());
	}
	let app = get_mut_app();
	app.save_state = parsed.unwrap();

	// initial setup
	app.module = Modules::get_module(app.save_state.module);
	app.module.load(&app.save_state);
	println!("-> {}", app.module.name());
}

pub(super) fn save_config() {
	let serialized = serde_json::to_string_pretty(&get_mut_app().save_state);
	if serialized.is_ok() {
		let output = std::fs::File::create(get_config_path().to_str().unwrap());
		if output.is_ok() {
			let _ = output.unwrap().write_all(serialized.unwrap().as_bytes());
		}
	}
}
#[macro_use]
extern crate oxide;

use oxide::{Mod, ModMetadata};

declare_mods!(ExampleMod);

pub struct ExampleMod;

impl oxide::Mod for ExampleMod {
    fn metadata(&self) -> ModMetadata {
        ModMetadata {
            id: "example".to_string(),
            name: "Example".to_string(),
            version: "1.0".to_string(),
        }
    }

    fn pre_init(&mut self) {
        println!("[ExampleMod] PreInit")
    }

    fn init(&mut self) {
        println!("[ExampleMod] Init")
    }

    fn post_init(&mut self) {
        println!("[ExampleMod] PostInit")
    }
}

#[macro_use]
extern crate oxide;

use oxide::ModMetadata;

declare_mods!(ExampleMod);

pub struct ExampleMod;

impl oxide::Mod for ExampleMod {
    fn metadata<'a>(&self) -> ModMetadata<'a> {
        ModMetadata {
            id: "example",
            name: "Example",
            version: "1.0",
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

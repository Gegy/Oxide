#![feature(const_fn)]
#![feature(concat_idents)]

extern crate jni;
#[macro_use]
extern crate lazy_static;
extern crate libloading;

pub mod java;
mod primitives;
mod arrays;
#[macro_use]
pub mod macros;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jobjectArray;
use libloading::{Library, Symbol};
use std::any::Any;
use std::sync::Mutex;

pub use java::*;
pub use macros::*;

lazy_static! {
    pub static ref MOD_REGISTRY: Mutex<ModRegistry> = Mutex::new(ModRegistry::new());
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_net_gegy1000_oxide_OxideNative_loadMod(env: JNIEnv, _: JClass, name: JString) {
    let name: String = env.get_string(name).expect("failed to get mod location").into();
    if let Err(e) = load_mod(&name) {
        println!("failed to load mod '{}': {:?}", name, e);
    }
}

fn load_mod(name: &String) -> Result<(), ModLoadError> {
    let library = Library::new(name)
        .map_err(|e| ModLoadError::Link(e))?;

    let mods: Vec<Box<Mod>> = collect_mods(&library)?;
    for modification in mods {
        MOD_REGISTRY.try_lock().unwrap().register(modification);
    }

    Ok(())
}

fn collect_mods(library: &Library) -> Result<Vec<Box<Mod>>, ModLoadError> {
    type CollectModsFunction = unsafe fn() -> Vec<Box<Mod>>;
    unsafe {
        let collect_symbol: Symbol<CollectModsFunction> = library.get(b"collect_mods")
            .map_err(|e| ModLoadError::Function(e))?;
        Ok(collect_symbol())
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_net_gegy1000_oxide_OxideNative_collectMetadata(env: JNIEnv, _: JClass) -> jobjectArray {
    let registry = MOD_REGISTRY.try_lock().unwrap();
    let mods = &registry.mods;
    let metadata: Vec<_> = mods.iter().map(|m| m.metadata.clone()).collect();
    metadata.to_java(&env).expect("failed to upload metadata").into_inner()
}

#[derive(Debug)]
pub enum ModLoadError {
    Link(std::io::Error),
    Function(std::io::Error),
}

pub struct ModRegistry {
    mods: Vec<ModContainer>,
}

impl ModRegistry {
    fn new() -> ModRegistry {
        ModRegistry { mods: Vec::new() }
    }

    fn register(&mut self, modification: Box<Mod>) {
        let metadata = modification.metadata();
        self.mods.push(ModContainer { modification, metadata });
    }
}

pub struct ModContainer {
    modification: Box<Mod>,
    metadata: ModMetadata,
}

pub trait Mod: Any + Send + Sync {
    fn metadata(&self) -> ModMetadata;

    fn pre_init(&mut self) {}

    fn init(&mut self) {}

    fn post_init(&mut self) {}
}

java_class! {
    #[derive(Clone, Debug)]
    pub struct ModMetadata("net/gegy1000/oxide/RustModMetadata") {
        pub id: String,
        pub name: String,
        pub version: String,
    }
}

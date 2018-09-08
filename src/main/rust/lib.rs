#![feature(const_fn)]
#![feature(concat_idents)]

extern crate jni;
#[macro_use]
extern crate lazy_static;
extern crate libloading;

pub use java::*;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::jobjectArray;
use libloading::{Library, Symbol};
pub use macros::*;
use std::any::Any;
use std::sync::Mutex;

pub mod java;
mod primitives;
mod arrays;
#[macro_use]
pub mod macros;

lazy_static! {
    pub static ref MOD_REGISTRY: Mutex<ModRegistry<'static>> = Mutex::new(ModRegistry::new());
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

    let mut registry = MOD_REGISTRY.try_lock().unwrap();

    let mods: Vec<Box<Mod>> = collect_mods(&library)?;
    for modification in mods {
        registry.register(modification);
    }

    registry.register_library(library);

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
    let metadata: Vec<_> = mods.iter().enumerate().map(|(i, m)| {
        let metadata = &m.metadata;
        JavaModMetadata {
            nativeId: i as i32,
            id: metadata.id.to_owned(),
            name: metadata.name.to_owned(),
            version: metadata.version.to_owned(),
        }
    }).collect();
    metadata.to_java(&env).expect("failed to upload metadata").into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_net_gegy1000_oxide_OxideNative_dispatchPreInit(_env: JNIEnv, _: JClass, native_id: i32, _event: JObject) {
    let mut registry = MOD_REGISTRY.try_lock().unwrap();
    if let Some(ref mut m) = registry.mods.get_mut(native_id as usize) {
        m.modification.pre_init();
    }
}

#[derive(Debug)]
pub enum ModLoadError {
    Link(std::io::Error),
    Function(std::io::Error),
}

pub struct ModRegistry<'a> {
    mods: Vec<ModContainer<'a>>,
    libraries: Vec<Library>,
}

impl<'a> ModRegistry<'a> {
    fn new() -> ModRegistry<'a> {
        ModRegistry { mods: Vec::new(), libraries: Vec::new() }
    }

    fn register(&mut self, modification: Box<Mod>) {
        let metadata = modification.metadata();
        self.mods.push(ModContainer { modification, metadata });
    }

    fn register_library(&mut self, library: Library) {
        self.libraries.push(library);
    }
}

pub struct ModContainer<'a> {
    modification: Box<Mod>,
    metadata: ModMetadata<'a>,
}

pub trait Mod: Any + Send + Sync {
    fn metadata<'a>(&self) -> ModMetadata<'a>;

    fn pre_init(&mut self) {}

    fn init(&mut self) {}

    fn post_init(&mut self) {}
}

#[derive(Clone, Debug)]
pub struct ModMetadata<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

java_class! {
    #[derive(Clone, Debug)]
    pub struct JavaModMetadata("net/gegy1000/oxide/RustModMetadata") {
        pub nativeId: i32,
        pub id: String,
        pub name: String,
        pub version: String,
    }
}

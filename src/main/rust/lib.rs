extern crate jni;
#[macro_use]
extern crate lazy_static;
extern crate libloading;

pub mod convert;
#[macro_use]
pub mod macros;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::jobjectArray;
use libloading::{Library, Symbol};
use std::any::Any;
use std::sync::Mutex;
use convert::{ToJava, JavaType};

pub use convert::*;
pub use macros::*;

// TODO: Note we can take a this instance too
// TODO: We can have a handler that throws Result::Err(E)

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
    let library = match Library::new(name) {
        Ok(l) => l,
        Err(e) => return Err(ModLoadError::Link(e)),
    };

    let mods: Vec<Box<Mod>> = collect_mods(&library)?;
    for modification in mods {
        MOD_REGISTRY.try_lock().unwrap().register(modification);
    }

    Ok(())
}

fn collect_mods(library: &Library) -> Result<Vec<Box<Mod>>, ModLoadError> {
    unsafe {
        let collect_symbol: Symbol<unsafe fn() -> Vec<Box<Mod>>> = match library.get(b"collect_mods") {
            Ok(f) => f,
            Err(e) => return Err(ModLoadError::Function(e)),
        };
        Ok(collect_symbol())
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_net_gegy1000_oxide_OxideNative_collectMetadata(env: JNIEnv, _: JClass) -> jobjectArray {
    let registry = MOD_REGISTRY.try_lock().unwrap();
    let mods = &registry.mods;
    let metadata: Vec<_> = mods.iter().map(|m| m.metadata.clone()).collect();
    metadata.to_java(&env)
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

#[derive(Clone, Debug)]
pub struct ModMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
}

impl<'a> ToJava<'a, JObject<'a>> for ModMetadata {
    fn to_java(&self, env: &JNIEnv<'a>) -> JObject<'a> {
        java_new! {
            net_gegy1000_oxide_RustModMetadata(env) {
                id: java_lang_String = self.id,
                name: java_lang_String = self.name,
                version: java_lang_String = self.version,
            }
        }
    }
}

java_type!(ModMetadata, "net/gegy1000/oxide/RustModMetadata");

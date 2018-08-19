extern crate jni;
extern crate libc;

use jni::{JNIEnv, sys::JNINativeInterface_};
use jni::objects::{JClass, JFieldID, JObject, JString, JValue};
use jni::strings::JNIStr;
use jni::sys::{jclass, jobject, jstring};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

// TODO: Note we can take a this instance too
// TODO: We can have a handler that throws Result::Err(E)

pub trait IntoJava<T> {
    fn into_java(self, env: &JNIEnv) -> T;
}

pub struct ModMetadata<'a> {
    pub modid: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

impl IntoJava<JObject> for ModMetadata {
    fn into_java(self, env: &JNIEnv) -> JObject {
        let object = env.new_object("net/gegy1000/oxide/RustModMetadata", "()V", &[]).expect("failed to create meta");

        let modid: JObject = self.modid.into_java(env);
        env.set_field(object, "modid", "Ljava/lang/String;", modid.into()).expect("failed to set modid field");

        let name: JObject = self.name.into_java(env);
        env.set_field(object, "name", "Ljava/lang/String;", name.into()).expect("failed to set name field");

        let version: JObject = self.version.into_java(env);
        env.set_field(object, "version", "Ljava/lang/String;", version.into()).expect("failed to set version field");

        object
    }
}

impl IntoJava<JObject> for &str {
    fn into_java(self, env: &JNIEnv) -> JObject {
        env.new_string(self).unwrap().into()
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_net_gegy1000_oxide_RustBootstrap_constructMod(env: JNIEnv, class: JClass) -> jobject {
    let metadata = ModMetadata {
        modid: "rustmod",
        name: "Rust Mod",
        version: "1.0.0",
    };
    metadata.into_java(&env).into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_net_gegy1000_oxide_RustBootstrap_onPreInit(env: JNIEnv, class: JClass, event: JObject) {
    println!("pre init from rust");
}

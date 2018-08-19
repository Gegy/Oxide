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

pub trait ToJava<'a, T> {
    fn to_java(&self, env: &JNIEnv<'a>) -> T;
}

pub struct ModMetadata<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

impl<'a> ToJava<'a, JObject<'a>> for ModMetadata<'a> {
    fn to_java(&self, env: &JNIEnv<'a>) -> JObject<'a> {
        let object = env.new_object("net/gegy1000/oxide/RustModMetadata", "()V", &[]).expect("failed to create meta");

        let id: JObject = self.id.to_java(env);
        env.set_field(object, "id", "Ljava/lang/String;", id.into()).expect("failed to set modid field");

        let name: JObject = self.name.to_java(env);
        env.set_field(object, "name", "Ljava/lang/String;", name.into()).expect("failed to set name field");

        let version: JObject = self.version.to_java(env);
        env.set_field(object, "version", "Ljava/lang/String;", version.into()).expect("failed to set version field");

        object
    }
}

impl<'a> ToJava<'a, JObject<'a>> for str {
    fn to_java(&self, env: &JNIEnv<'a>) -> JObject<'a> {
        env.new_string(self).unwrap().into()
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_net_gegy1000_oxide_RustBootstrap_constructMod(env: JNIEnv, class: JClass) -> jobject {
    let metadata = ModMetadata {
        id: "rustmod",
        name: "Rust Mod",
        version: "1.0.0",
    };
    metadata.to_java(&env).into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_net_gegy1000_oxide_RustBootstrap_onPreInit(env: JNIEnv, class: JClass, event: JObject) {
    println!("pre init from rust");
}

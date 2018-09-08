use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::signature::JavaType;
use std::cell::UnsafeCell;
use std::fmt;
use std::ops::{Deref, DerefMut};

pub type JavaResult<T> = ::jni::errors::Result<T>;

pub trait JavaAlias {
    fn java_alias() -> JavaType;
}

pub trait JavaValue<'a> {}

pub trait IntoJValue<'a> {
    fn into_value(self) -> JValue<'a>;
}

pub trait UnwrapJValue<'a, T> {
    fn unwrap_value(self) -> T;
}

pub trait JavaStruct<'a>: JavaAlias + Sized {
    fn construct(env: &'a JNIEnv<'a>) -> JavaResult<JObject<'a>> {
        if let JavaType::Object(class) = Self::java_alias() {
            env.new_object(class, "()V", &[])
        } else {
            panic!("struct alias was not object");
        }
    }

    fn serialize(&self, env: &'a JNIEnv<'a>, target: &mut JObject<'a>) -> JavaResult<()>;

    fn deserialize(env: &'a JNIEnv<'a>, source: &JObject<'a>) -> JavaResult<Self>;
}

pub trait FromJava<'a, J: JavaValue<'a>>: Sized {
    fn from_java(env: &'a JNIEnv<'a>, value: J) -> JavaResult<Self>;
}

pub trait ToJava<'a, J: JavaValue<'a> + Sized> {
    fn to_java(&self, env: &'a JNIEnv<'a>) -> JavaResult<J>;
}

pub struct JavaBox<'a, T: JavaStruct<'a>> {
    env: &'a JNIEnv<'a>,
    inner: JObject<'a>,
    resolved: UnsafeCell<Option<T>>,
}

impl<'a, T: JavaStruct<'a>> JavaBox<'a, T> {
    pub fn new(env: &'a JNIEnv<'a>, value: T) -> JavaResult<JavaBox<'a, T>> {
        let mut object = T::construct(env)?;
        value.serialize(env, &mut object)?;
        Ok(JavaBox {
            env,
            inner: object,
            resolved: UnsafeCell::new(Some(value)),
        })
    }

    fn borrow_mut(&self) -> &mut T {
        let slot = unsafe { &mut *self.resolved.get() };
        if slot.is_none() {
            let parsed = T::deserialize(self.env, &self.inner)
                .expect("failed to deserialize java value");
            *slot = Some(parsed);
        }
        slot.as_mut().unwrap()
    }
}

impl<'a, T: JavaStruct<'a>> Deref for JavaBox<'a, T> {
    type Target = T;

    fn deref(&self) -> &T { self.borrow_mut() }
}

impl<'a, T: JavaStruct<'a>> DerefMut for JavaBox<'a, T> {
    fn deref_mut(&mut self) -> &mut T { self.borrow_mut() }
}

impl<'a, T: JavaStruct<'a>> Drop for JavaBox<'a, T> {
    fn drop(&mut self) {
        if let Some(ref resolved) = unsafe { &*self.resolved.get() } {
            resolved.serialize(self.env, &mut self.inner)
                .expect("failed to upload java type to jni");
        }
    }
}

impl<'a, T: JavaStruct<'a>> FromJava<'a, JObject<'a>> for JavaBox<'a, T> {
    fn from_java(env: &'a JNIEnv<'a>, value: JObject<'a>) -> JavaResult<Self> {
        Ok(JavaBox {
            env,
            inner: value,
            resolved: UnsafeCell::new(None),
        })
    }
}

impl<'a, T: JavaStruct<'a>> ToJava<'a, JObject<'a>> for JavaBox<'a, T> {
    fn to_java(&self, _: &'a JNIEnv<'a>) -> JavaResult<JObject<'a>> { Ok(self.inner) }
}

impl<'a, T: JavaStruct<'a>> JavaAlias for JavaBox<'a, T> {
    fn java_alias() -> JavaType { T::java_alias() }
}

impl<'a, T: JavaStruct<'a> + fmt::Debug> fmt::Debug for JavaBox<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T: JavaStruct<'a>> ToJava<'a, JObject<'a>> for T {
    fn to_java(&self, env: &'a JNIEnv<'a>) -> JavaResult<JObject<'a>> {
        let mut object = T::construct(env)?;
        self.serialize(env, &mut object)?;
        Ok(object)
    }
}

impl<'a, T: JavaStruct<'a>> FromJava<'a, JObject<'a>> for T {
    fn from_java(env: &'a JNIEnv<'a>, value: JObject<'a>) -> JavaResult<T> {
        T::deserialize(env, &value)
    }
}

impl<'a> JavaValue<'a> for JObject<'a> {}

impl<'a> IntoJValue<'a> for JObject<'a> {
    fn into_value(self) -> JValue<'a> { JValue::Object(self) }
}

impl<'a> UnwrapJValue<'a, JObject<'a>> for JValue<'a> {
    fn unwrap_value(self) -> JObject<'a> {
        match self {
            JValue::Object(o) => o,
            _ => panic!("failed to unwrap JValue"),
        }
    }
}

impl JavaAlias for String {
    fn java_alias() -> JavaType { JavaType::Object("java/lang/String".to_string()) }
}

impl<'a> FromJava<'a, JObject<'a>> for String {
    fn from_java(env: &'a JNIEnv<'a>, value: JObject<'a>) -> JavaResult<Self> {
        env.get_string(value.into()).map(|s| s.into())
    }
}

impl<'a> ToJava<'a, JObject<'a>> for String {
    fn to_java(&self, env: &'a JNIEnv<'a>) -> JavaResult<JObject<'a>> {
        env.new_string(self.clone()).map(|s| s.into())
    }
}

use ::java::{FromJava, IntoJValue, JavaAlias, JavaResult, JavaValue, ToJava, UnwrapJValue};
use jni::JNIEnv;
use jni::objects::JValue;
use jni::signature::{JavaType, Primitive};
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};

macro_rules! impl_java_value {
    (
        $(
            $variant:path => $T:ty;
        )*
    ) => {
        $(
            impl<'a> JavaValue<'a> for $T {}

            impl<'a> IntoJValue<'a> for $T {
                fn into_value(self) -> JValue<'a> { $variant(self) }
            }

            impl<'a> UnwrapJValue<'a, $T> for JValue<'a> {
                fn unwrap_value(self) -> $T {
                    match self {
                        $variant(v) => v,
                        _ => panic!("failed to unwrap JValue"),
                    }
                }
            }
        )*
    }
}

macro_rules! impl_primitive_convert {
    (
        $(
            $T:ty => $J:ty,
        )*
    ) => {
        $(
            impl<'a> FromJava<'a, $J> for $T {
                fn from_java(_: &'a JNIEnv<'a>, value: $J) -> JavaResult<Self> {
                    Ok(value)
                }
            }

            impl<'a> ToJava<'a, $J> for $T {
                fn to_java(&self, _: &'a JNIEnv<'a>) -> JavaResult<$J> {
                    Ok(*self as $T)
                }
            }
        )*
    };
}

macro_rules! impl_primitive_alias {
    (
        $(
            $T:ty => $val:expr,
        )*
    ) => {
        $(
            impl JavaAlias for $T {
                fn java_alias() -> JavaType { JavaType::Primitive($val) }
            }
        )*
    }
}

impl_java_value! {
    JValue::Byte => jbyte;
    JValue::Short => jshort;
    JValue::Int => jint;
    JValue::Long => jlong;
    JValue::Float => jfloat;
    JValue::Double => jdouble;
    JValue::Bool => jboolean;
    JValue::Char => jchar;
}

impl_primitive_alias! {
    i8 => Primitive::Byte,
    i16 => Primitive::Short,
    i32 => Primitive::Int,
    i64 => Primitive::Long,
    f32 => Primitive::Float,
    f64 => Primitive::Double,
    bool => Primitive::Boolean,
    char => Primitive::Char,
}

impl_primitive_convert! {
    i8 => jbyte,
    i16 => jshort,
    i32 => jint,
    i64 => jlong,
    f32 => jfloat,
    f64 => jdouble,
}

impl<'a> FromJava<'a, jboolean> for bool {
    fn from_java(_: &'a JNIEnv<'a>, value: jboolean) -> JavaResult<Self> { Ok(value != 0) }
}

impl<'a> ToJava<'a, jboolean> for bool {
    fn to_java(&self, _: &'a JNIEnv<'a>) -> JavaResult<jboolean> { Ok(*self as jboolean) }
}

impl<'a> FromJava<'a, jchar> for char {
    fn from_java(_: &'a JNIEnv<'a>, value: jchar) -> JavaResult<char> {
        Ok(::std::char::from_u32(value as u32).unwrap())
    }
}

impl<'a> ToJava<'a, jchar> for char {
    fn to_java(&self, _: &'a JNIEnv<'a>) -> JavaResult<jchar> { Ok(*self as jchar) }
}

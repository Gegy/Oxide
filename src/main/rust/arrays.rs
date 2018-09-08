use ::java::{FromJava, JavaAlias, JavaResult, ToJava};
use jni::JNIEnv;
use jni::objects::JObject;
use jni::signature::JavaType;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use jni::sys::{jbooleanArray, jbyteArray, jcharArray, jdoubleArray, jfloatArray, jintArray, jlongArray, jobjectArray, jshortArray};

impl<'a, T: JavaAlias> JavaAlias for Vec<T> {
    fn java_alias() -> JavaType { JavaType::Array(Box::new(T::java_alias())) }
}

impl<'a, T> FromJava<'a, JObject<'a>> for Vec<T>
    where T: JavaAlias + ToJava<'a, JObject<'a>> + FromJava<'a, JObject<'a>>
{
    fn from_java(env: &'a JNIEnv<'a>, value: JObject<'a>) -> JavaResult<Self> {
        let array: jobjectArray = value.into_inner() as jobjectArray;
        let len = env.get_array_length(array)?;
        let mut result = Vec::with_capacity(len as usize);
        for i in 0..len {
            let object = env.get_object_array_element(array, i)?;
            result.push(T::from_java(env, object)?);
        }
        Ok(result)
    }
}

impl<'a, T> ToJava<'a, JObject<'a>> for Vec<T>
    where T: JavaAlias + ToJava<'a, JObject<'a>> + FromJava<'a, JObject<'a>>
{
    fn to_java(&self, env: &'a JNIEnv<'a>) -> JavaResult<JObject<'a>> {
        if let JavaType::Object(class) = T::java_alias() {
            let len = self.len() as i32;
            let array: jobjectArray = env.new_object_array(len, class, JObject::null())?;
            for i in 0..len {
                if let Some(element) = self.get(i as usize) {
                    let java: JObject<'a> = element.to_java(env)?;
                    env.set_object_array_element(array, i, java)?;
                }
            }
            Ok(array.into())
        } else {
            panic!("tried to convert non-object array into object array")
        }
    }
}

macro_rules! impl_primitive_array_convert {
    ($T:ty, $A:ty, $new:ident, $set:ident, $get:ident) => {
        impl<'a> FromJava<'a, JObject<'a>> for Vec<$T> {
            fn from_java(env: &'a JNIEnv<'a>, value: JObject<'a>) -> JavaResult<Self> {
                let array: $A = value.into_inner() as $A;
                let mut result = Vec::with_capacity(env.get_array_length(array)? as usize);
                env.$get(array, 0, &mut result)?;
                Ok(result)
            }
        }

        impl<'a> ToJava<'a, JObject<'a>> for Vec<$T> {
            fn to_java(&self, env: &'a JNIEnv<'a>) -> JavaResult<JObject<'a>> {
                let len = self.len() as i32;
                let array: $A = env.$new(len)?;
                env.$set(array, 0, &self)?;
                Ok(array.into())
            }
        }
    };
}

impl_primitive_array_convert!(jbyte, jbyteArray, new_byte_array, set_byte_array_region, get_byte_array_region);
impl_primitive_array_convert!(jshort, jshortArray, new_short_array, set_short_array_region, get_short_array_region);
impl_primitive_array_convert!(jint, jintArray, new_int_array, set_int_array_region, get_int_array_region);
impl_primitive_array_convert!(jlong, jlongArray, new_long_array, set_long_array_region, get_long_array_region);
impl_primitive_array_convert!(jfloat, jfloatArray, new_float_array, set_float_array_region, get_float_array_region);
impl_primitive_array_convert!(jdouble, jdoubleArray, new_double_array, set_double_array_region, get_double_array_region);
impl_primitive_array_convert!(jboolean, jbooleanArray, new_boolean_array, set_boolean_array_region, get_boolean_array_region);
impl_primitive_array_convert!(jchar, jcharArray, new_char_array, set_char_array_region, get_char_array_region);


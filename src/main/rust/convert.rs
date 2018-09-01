use jni::{JNIEnv, objects::JObject, sys::jobjectArray};

pub trait ToJava<'a, T> {
    fn to_java(&self, env: &JNIEnv<'a>) -> T;
}

pub trait JavaType {
    fn java_type() -> &'static str;
}

impl<'a> ToJava<'a, JObject<'a>> for str {
    fn to_java(&self, env: &JNIEnv<'a>) -> JObject<'a> {
        env.new_string(self).expect("failed to create java string").into()
    }
}

impl<'a, T> ToJava<'a, jobjectArray> for Vec<T>
    where T: ToJava<'a, JObject<'a>> + JavaType
{
    fn to_java(&self, env: &JNIEnv<'a>) -> jobjectArray {
        let array_type = env.find_class(T::java_type()).expect("failed to find array type class");
        let java_array: jobjectArray = env.new_object_array(self.len() as i32, array_type, JObject::null())
            .expect("failed to create array");

        for (i, element) in self.iter().enumerate() {
            env.set_object_array_element(java_array, i as i32, element.to_java(env))
                .expect("failed to set object array element");
        }

        java_array
    }
}

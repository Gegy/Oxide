#[macro_export]
macro_rules! declare_mods {
    ($($mods:expr),*) => {
        #[no_mangle]
        pub extern "C" fn collect_mods() -> Vec<Box<$crate::Mod>> {
            let mut mods: Vec<Box<$crate::Mod>> = Vec::new();
            $(
                mods.push(Box::new($mods));
            )*
            mods
        }
    };
}

// TODO: Duplication
#[macro_export]
macro_rules! java_class {
    (
        $(#[$outer:meta])*
        pub struct $T:ident<$lt:tt>($path:expr) {
            $(
                pub $f_name:ident: $f_ty:ty,
            )*
        }
    ) => {
        $(#[$outer])*
        pub struct $T<$lt> {
            $(
                pub $f_name: $f_ty,
            )*
            _phantom: ::std::marker::PhantomData<&$lt ()>,
        }

        impl<$lt> $T<$lt> {
            pub fn new($($f_name: $f_ty),*) -> Self {
                $T {
                    $(
                        $f_name,
                    )*
                    _phantom: ::std::marker::PhantomData,
                }
            }
        }

        impl<$lt> $crate::JavaAlias for $T<$lt> {
            fn java_alias() -> ::jni::signature::JavaType {
                ::jni::signature::JavaType::Object($path.to_string())
            }
        }

        impl<$lt> $crate::JavaStruct<$lt> for $T<$lt> {
            fn serialize(&self, env: &$lt ::jni::JNIEnv<$lt>, target: &mut ::jni::objects::JObject<$lt>) -> $crate::JavaResult<()> {
                use $crate::java::ToJava;
                $(
                    {
                        let value: ::jni::objects::JValue = self.$f_name.to_java(env)?.into_value();
                        env.set_field(*target, stringify!($f_name), <$f_ty>::java_alias().to_string(), value)?;
                    }
                )*
                Ok(())
            }

            fn deserialize(env: &$lt ::jni::JNIEnv<$lt>, source: &::jni::objects::JObject<$lt>) -> $crate::JavaResult<Self> {
                Ok($T {
                    $(
                        $f_name: <$f_ty>::from_java(
                            env,
                            env.get_field(*source, stringify!($f_name), <$f_ty>::java_alias().to_string())?.unwrap_value()
                        )?,
                    )*
                    _phantom: ::std::marker::PhantomData,
                })
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub struct $T:ident($path:expr) {
            $(
                pub $f_name:ident: $f_ty:ty,
            )*
        }
    ) => {
        $(#[$outer])*
        pub struct $T {
            $(
                pub $f_name: $f_ty,
            )*
        }

        impl $T {
            pub fn new($($f_name: $f_ty),*) -> Self {
                $T {
                    $($f_name),*
                }
            }
        }

        impl $crate::JavaAlias for $T {
            fn java_alias() -> ::jni::signature::JavaType {
                ::jni::signature::JavaType::Object($path.to_string())
            }
        }

        impl<'a> $crate::JavaStruct<'a> for $T {
            fn serialize(&self, env: &'a ::jni::JNIEnv<'a>, target: &mut ::jni::objects::JObject<'a>) -> $crate::JavaResult<()> {
                use $crate::java::ToJava;
                $(
                    {
                        let value: ::jni::objects::JValue = self.$f_name.to_java(env)?.into_value();
                        env.set_field(*target, stringify!($f_name), <$f_ty>::java_alias().to_string(), value)?;
                    }
                )*
                Ok(())
            }

            fn deserialize(env: &'a ::jni::JNIEnv<'a>, source: &::jni::objects::JObject<'a>) -> $crate::JavaResult<Self> {
                Ok($T {
                    $(
                        $f_name: <$f_ty>::from_java(
                            env,
                            env.get_field(*source, stringify!($f_name), <$f_ty>::java_alias().to_string())?.unwrap_value()
                        )?,
                    )*
                })
            }
        }
    };
}

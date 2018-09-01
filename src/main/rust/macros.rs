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

#[macro_export]
macro_rules! java_type {
    (
        $T:ty, $path:expr
    ) => (
        impl $crate::JavaType for $T {
            fn java_type() -> &'static str { $path }
        }
    )
}

#[macro_export]
macro_rules! java_new {
    (
        $path:ident($env:expr) {
            $(
                $name:ident: $desc:ident = $value:expr,
            )*
        }
    ) => (
        let object = $env.new_object(path!($path), "()V", &[])
            .expect(format!("failed to create object '{}'", stringify!($path)).as_str());
        $(
            let value: JObject = $value.to_java($env);
            $env.set_field(object, stringify!($name), desc!($desc), value.into())
                .expect(format!("failed to set field '{}'", stringify!($name)).as_str());
        )*
        object
    );
}

macro_rules! desc {
    ($path:ident) => (
        format!("L{};", path!($path))
    )
}

macro_rules! path {
    ($path:ident) => (
        stringify!($path).replace("_", "/").as_str()
    )
}

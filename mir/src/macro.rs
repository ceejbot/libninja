/// import!("bytes", a, b, c);
/// import!("bytes");
/// import!("bytes::*");
/// import!(pub "bytes::*");
#[macro_export]
macro_rules! import {
    ($path:literal) => {
        ::mir::Import::package($path)
    };
    ($path:literal, $($imports:ident),*) => {
        ::mir::Import::new($path, vec![$(stringify!($imports)),*])
    };
    (pub $path:literal, $($imports:ident),*) => {
        ::mir::Import::new($path, vec![$(stringify!($imports)),*]).public()
    };
    ($path:path, $($imports:ident),*) => {
        ::mir::Import::new(stringify!($path), vec![$(stringify!($imports)),*])
    };
    (pub $path:path, $($imports:ident),*) => {
        ::mir::Import::new(stringify!($path), vec![$(stringify!($imports)),*]).public()
    };
    ($c:expr, $($imports:expr),*) => {
        ::mir::Import::new($c, vec![$($imports),*])
    };
    ($c:expr) => {
        ::mir::Import::package($c)
    };
}
/// Macro to create a FnArg. Called targ because the type is a TokenStream (specifically a path), rather than a &str.
/// targ!(access_token: String)
#[macro_export]
macro_rules! targ {
    ($name:ident : $ty:path) => {
        ::mir::FnArg {
            name: stringify!($name).to_string(),
            ty: {
                let ty = ::syn::parse_str::<::syn::Ident>(stringify!($ty)).unwrap();
                ::quote::ToTokens::to_token_stream(&ty)
            },
            default: None,
            treatment: None,
        }
    };
}
/// Macro to create a FnArg. Second arg is a expr that evaluates to a string. Optionally takes a default.
/// arg!(count: "int" = 500)
#[macro_export]
macro_rules! arg {
    ($name:ident : $ty:expr) => {
        ::mir::FnArg {
            name: ::mir::ArgIdent::from(stringify!($name)),
            ty: $ty.to_string(),
            default: None,
            treatment: None,
        }
    };
    ($name:ident : $ty:expr , $default:expr) => {
        ::mir::FnArg {
            name: ::mir::ArgIdent::from(stringify!($name).to_string()),
            ty: $ty.to_string(),
            default: $default.to_string(),
            treatment: None,
        }
    };
}

/// A literal value.
#[macro_export]
macro_rules! lit {
    ($($arg:tt)*) => {
        ::mir::Literal(format!($($arg)*));
    };
}

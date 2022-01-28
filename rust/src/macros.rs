macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )*
    };
}

macro_rules! create_http {
    ($($service:expr),+) => {
        HttpServer::new(|| {
            App::new()
            $(
                .service($service)
            )*
        })
    }
}

macro_rules! count {
    () => { 0usize };
    ($a:expr, $($b:expr),*) => { 1usize + count!($($b,)*) };
    ($a:item, $($b:item),*) => { 1usize + count!($($b,)*) };
    ($a:block, $($b:block),*) => { 1usize + count!($($b,)*) };
    ($a:stmt, $($b:stmt),*) => { 1usize + count!($($b,)*) };
    ($a:pat_param, $($b:pat_param),*) => { 1usize + count!($($b,)*) };
    ($a:pat, $($b:pat),*) => { 1usize + count!($($b,)*) };
    ($a:ty, $($b:ty),*) => { 1usize + count!($($b,)*) };
    ($a:ident, $($b:ident),*) => { 1usize + count!($($b,)*) };
    ($a:path, $($b:path),*) => { 1usize + count!($($b,)*) };
    ($a:tt, $($b:tt),*) => { 1usize + count!($($b,)*) };
    ($a:meta, $($b:meta),*) => { 1usize + count!($($b,)*) };
    ($a:lifetime, $($b:lifetime),*) => { 1usize + count!($($b,)*) };
    ($a:vis, $($b:vis),*) => { 1usize + count!($($b,)*) };
    ($a:literal, $($b:literal),*) => { 1usize + count!($($b,)*) };
}
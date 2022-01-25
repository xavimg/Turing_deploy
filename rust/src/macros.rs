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
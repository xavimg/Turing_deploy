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

            // LOG REQUESTS & RESPONSES
            .wrap_fn(|req, srv| {
                CURRENT_LOGGER.async_log_info(format!("{req:?}"));
                let fut = srv.call(req);
                async {
                    match fut.await {
                        Err(e) => { CURRENT_LOGGER.async_log_error(format!("{e:?}")); Err(e) },
                        Ok(x) => {
                            if x.status().is_success() {
                                CURRENT_LOGGER.async_log_info(format!("RESP {x:?}"))
                            } else {
                                CURRENT_LOGGER.async_log_warning(format!("RESP {x:?}"))
                            }
                            Ok(x)
                        }
                    }
                }
            })

            // ADD SERVICES
            $(
                .service($service)
            )*
        })
    }
}

macro_rules! get_env {
    ($key:expr) => {
        {
            match option_env!($key) {
                Some(x) => x.to_string(),
                None => match std::env::var($key) {
                    Err(x) => panic!("{x}"),
                    Ok(x) => x
                }
            }
        }
    };
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
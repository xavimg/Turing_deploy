flat_mod!(logger, color, seq_await, math, serdex, randx, array_map, take_out, durationx);

pub unsafe fn upgrade<T> (ptr: &T) -> &mut T {
    let ptr = ptr as *const T as *mut T;
    &mut *ptr
}

pub fn trim (str: impl Into<String>) -> String {
    let str = str.into();
    let mut chars = str.chars();
    let mut last = chars.next().unwrap();

    let mut result = String::with_capacity(str.len());
    while let Some(c) = chars.next() {
        if c == '\n' || c == '\r' {
            continue
        }

        if last.is_whitespace() && c.is_whitespace() {
            last = c;
            continue
        }

        result.push(last)
    }

    if !last.is_whitespace() { result.push(last); }
    result
}
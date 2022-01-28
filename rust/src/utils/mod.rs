flat_mod!(color, seq_await, math, serdex, randx, array_map, take_out, durationx);

pub unsafe fn upgrade<T> (ptr: &T) -> &mut T {
    let ptr = ptr as *const T as *mut T;
    &mut *ptr
}
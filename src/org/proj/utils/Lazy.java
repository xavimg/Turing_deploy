package org.proj.utils;

import java.util.Objects;
import java.util.function.Supplier;

final public class Lazy<T> {
    private T value;
    final private Supplier<T> supplier;

    public Lazy (Supplier<T> supplier) {
        Objects.requireNonNull(supplier);
        this.value = null;
        this.supplier = supplier;
    }

    public Lazy (T value) {
        Objects.requireNonNull(value);
        this.value = value;
        this.supplier = null;
    }

    public boolean isComputed () {
        return value != null;
    }

    public T get () {
        if (value == null) {
            assert supplier != null;
            value = supplier.get();
            Objects.requireNonNull(value);
        }

        return value;
    }
}

package org.proj.utils;

import kotlin.Lazy;

import java.util.Optional;
import java.util.function.Supplier;

public class JavaLazy<T> implements Lazy<T> {
    private Optional<T> value;
    final private Supplier<T> supplier;

    public JavaLazy (Supplier<T> supplier) {
        this.value = Optional.empty();
        this.supplier = supplier;
    }

    public JavaLazy (T value) {
        this.value = Optional.of(value);
        this.supplier = null;
    }

    @Override
    public T getValue () {
        if (value.isEmpty()) {
            value = Optional.of(this.supplier.get());
        }

        return value.get();
    }

    @Override
    public boolean isInitialized() {
        return value.isPresent();
    }
}

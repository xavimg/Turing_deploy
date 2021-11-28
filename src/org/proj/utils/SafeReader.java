package org.proj.utils;

import org.proj.utils.chars.CharConsumer;
import org.proj.utils.chars.CharPredicate;

import java.io.Reader;
import java.util.Iterator;
import java.util.Optional;

public class SafeReader implements Iterator<Character> {
    final private Reader parent;
    private Optional<Character> cache;

    public SafeReader (Reader parent) {
        this.parent = parent;
    }

    @Override
    public boolean hasNext () {
        if (cache == null) {
            cache = safeNext();
        }

        return cache.isPresent();
    }

    @Override
    public Character next() {
        return safeNext().orElse(null);
    }

    public Optional<Character> safeNext() {
        if (cache != null) {
            Optional<Character> value = cache;
            cache = null;
            return value;
        }

        int read = -1;
        try {
            read = parent.read();
        } catch (Exception ignore) {}

        if (read == -1) {
            return Optional.empty();
        }

        return Optional.of((char) read);
    }

    public Optional<Character> skipWhile (boolean skipLast, CharPredicate predicate) {
        Optional<Character> next;
        while ((next = this.safeNext()).isPresent() && predicate.test(next.get())) {}
        if (!skipLast) cache = next;
        return next;
    }

    public String joinWhile (boolean skipLast, CharPredicate predicate) {
        Optional<Character> next;
        StringBuilder builder = new StringBuilder();

        while ((next = this.safeNext()).isPresent() && predicate.test(next.get())) builder.append(next.get());
        if (!skipLast) cache = next;
        return builder.toString();
    }

    public Optional<Character> forEachWhile (boolean skipLast, CharPredicate predicate, CharConsumer consumer) {
        Optional<Character> next;
        while ((next = this.safeNext()).isPresent() && predicate.test(next.get())) {
            consumer.accept(next.get());
        }

        if (!skipLast) cache = next;
        return next;
    }

    public void forEachRemaining (CharConsumer consumer) {
        Optional<Character> next;
        while ((next = this.safeNext()).isPresent()) {
            consumer.accept(next.get());
        }
    }
}

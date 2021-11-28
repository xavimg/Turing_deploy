package org.proj.utils.chars;

import java.util.Objects;
import java.util.PrimitiveIterator;
import java.util.function.IntConsumer;

public interface CharIterator extends PrimitiveIterator<Character, CharConsumer> {
    char nextChar ();

    @Override
    default Character next() {
        return nextChar();
    }

    default void forEachRemaining (CharConsumer action) {
        Objects.requireNonNull(action);
        while (hasNext())
            action.accept(nextChar());
    }
}

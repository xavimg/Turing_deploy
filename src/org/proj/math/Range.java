package org.proj.math;

import java.util.PrimitiveIterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.function.IntSupplier;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public class Range {
    public static IntStream ofInt (int from, int to) {
        PrimitiveIterator.OfInt iter = new PrimitiveIterator.OfInt() {
            int i = from;

            @Override
            public int nextInt() {
                return i++;
            }

            @Override
            public boolean hasNext() {
                return i < to;
            }
        };

        Spliterator.OfInt spliter = Spliterators.spliterator(iter, to - from + 1, Spliterator.ORDERED);
        return StreamSupport.intStream(spliter, false);
    }

    public static IntStream parallelOfInt (int from, int to) {
        PrimitiveIterator.OfInt iter = new PrimitiveIterator.OfInt() {
            int i = from;

            @Override
            public int nextInt() {
                return i++;
            }

            @Override
            public boolean hasNext() {
                return i < to;
            }
        };

        Spliterator.OfInt spliter = Spliterators.spliterator(iter, to - from, Spliterator.ORDERED);
        return StreamSupport.intStream(spliter, true);
    }
}

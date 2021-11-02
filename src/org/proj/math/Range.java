package org.proj.math;

import java.util.PrimitiveIterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.function.IntSupplier;
import java.util.stream.DoubleStream;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public class Range {
    public static <T> Stream<T> ofIterable (Iterable<T> iter, boolean parallel) {
        return StreamSupport.stream(iter.spliterator(), parallel);
    }

    public static IntStream ofInt (int from, int to, boolean parallel) {
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
        return StreamSupport.intStream(spliter, parallel);
    }

    public static DoubleStream ofDouble (double from, double to, double step, boolean parallel) {
        PrimitiveIterator.OfDouble iter = new PrimitiveIterator.OfDouble() {
            double x = from;

            @Override
            public double nextDouble() {
                double y = x;
                x += step;

                return y;
            }

            @Override
            public boolean hasNext() {
                return x < to;
            }
        };

        long size = (long) Math.ceil((to - from) / step);
        Spliterator.OfDouble spliter = Spliterators.spliterator(iter, size, Spliterator.ORDERED);
        return StreamSupport.doubleStream(spliter, parallel);
    }
}

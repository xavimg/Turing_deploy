package org.proj.math;

import org.proj.math.numbers.Complex;

import java.util.*;
import java.util.function.*;
import java.util.stream.*;

public class Range {
    public static <T> Stream<T> ofArray (T[] iter, boolean parallel) {
        return StreamSupport.stream(Arrays.spliterator(iter), parallel);
    }

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

    public static LongStream ofLong (long from, long to, boolean parallel) {
        PrimitiveIterator.OfLong iter = new PrimitiveIterator.OfLong() {
            long i = from;

            @Override
            public long nextLong() {
                return i++;
            }

            @Override
            public boolean hasNext() {
                return i < to;
            }
        };

        Spliterator.OfLong spliter = Spliterators.spliterator(iter, to - from, Spliterator.ORDERED);
        return StreamSupport.longStream(spliter, parallel);
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

    public static double sum (int from, int to, IntToDoubleFunction function) {
        PrimitiveIterator.OfDouble iter = new PrimitiveIterator.OfDouble() {
            int i = from;

            @Override
            public double nextDouble() {
                return function.applyAsDouble(i++);
            }

            @Override
            public boolean hasNext() {
                return i <= to;
            }
        };

        Spliterator.OfDouble split = Spliterators.spliterator(iter, to - from, 0);
        return StreamSupport.doubleStream(split, true).sum();
    }

    public static Complex sum (int from, int to, Function<Integer, Complex> function) {
        Iterator<Complex> iter = new Iterator<Complex>() {
            int i = from;

            @Override
            public Complex next() {
                return function.apply(i++);
            }

            @Override
            public boolean hasNext() {
                return i <= to;
            }
        };

        Spliterator<Complex> split = Spliterators.spliterator(iter, to - from, 0);
        return StreamSupport.stream(split, true).reduce(Complex::add).get();
    }
}

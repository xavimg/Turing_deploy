package org.proj.math;

import java.util.PrimitiveIterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.function.DoubleUnaryOperator;
import java.util.function.IntToDoubleFunction;
import java.util.stream.StreamSupport;

public class MathUtils {
    final public static double PI_2 = 2 * Math.PI;
    final public static double PI2 = Math.PI * Math.PI;
    final public static double HALF_PI = Math.PI / 2;

    public static double sum (int size, IntToDoubleFunction function) {
        return Range.ofInt(0, size, true).mapToDouble(function).sum();
    }

    public static double clamp (double value, double min, double max) {
        return Math.min(max, Math.max(min, value));
    }

    public static float clamp (float value, float min, float max) {
        return Math.min(max, Math.max(min, value));
    }

    public static int clamp (int value, int min, int max) {
        return Math.min(max, Math.max(min, value));
    }

    public static double integral (double from, double to, long epochs, DoubleUnaryOperator function) {
        double dist = to - from;
        double step = dist / epochs;

        PrimitiveIterator.OfDouble iter = new PrimitiveIterator.OfDouble() {
            double x = from;

            @Override
            public double nextDouble() {
                return function.applyAsDouble(x++) * step;
            }

            @Override
            public boolean hasNext() {
                return x <= to;
            }
        };

        Spliterator.OfDouble spliter = Spliterators.spliterator(iter, epochs, Spliterator.ORDERED);
        return StreamSupport.doubleStream(spliter, true).sum();
    }
}

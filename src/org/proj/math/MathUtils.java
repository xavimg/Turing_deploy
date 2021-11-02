package org.proj.math;

import org.proj.math.matrix.Matrix;

import java.math.BigDecimal;
import java.math.MathContext;
import java.util.PrimitiveIterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.function.DoubleUnaryOperator;
import java.util.function.IntToDoubleFunction;
import java.util.function.UnaryOperator;
import java.util.stream.StreamSupport;

public class MathUtils {
    final public static double PI_2 = 2 * Math.PI;
    final public static double PI2 = Math.PI * Math.PI;
    final public static double HALF_PI = Math.PI / 2;

    final private static BigDecimal DOUBLE_DELTA = BigDecimal.ONE.scaleByPowerOfTen(-16);

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

    public static float derivative (double x, DoubleUnaryOperator function) {
        return (float) ((function.applyAsDouble(x + 1e-7) - function.applyAsDouble(x)) / 1e-7d);
    }

    public static double derivative (BigDecimal x, UnaryOperator<BigDecimal> function) {
        return function.apply(x.add(DOUBLE_DELTA)).subtract(function.apply(x)).divide(DOUBLE_DELTA, MathContext.DECIMAL128).doubleValue();
    }

    public static double derivative (double x, UnaryOperator<BigDecimal> function) {
        return derivative(BigDecimal.valueOf(x), function);
    }
}

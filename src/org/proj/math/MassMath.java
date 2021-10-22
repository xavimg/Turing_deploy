package org.proj.math;

import java.math.BigDecimal;
import java.math.MathContext;
import java.util.function.DoubleFunction;
import java.util.function.DoubleUnaryOperator;
import java.util.function.IntFunction;
import java.util.function.IntToDoubleFunction;

public class MassMath {
    public static double sum (int size, IntToDoubleFunction function) {
        return Range.parallelOfInt(0, size).mapToDouble(function).sum();
    }

    public static BigDecimal sum (int size, IntFunction<BigDecimal> function) {
        return Range.parallelOfInt(0, size).mapToObj(function).reduce(BigDecimal::add).get();
    }
}

package org.proj.math;

import java.util.function.DoubleFunction;
import java.util.function.DoubleUnaryOperator;
import java.util.function.IntToDoubleFunction;

public class MassMath {
    public static double sum (int size, IntToDoubleFunction function) {
        return Range.parallelOfInt(0, size).mapToDouble(function).sum();
    }
}

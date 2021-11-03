package org.proj.math;

import org.proj.math.numbers.Chungus;
import org.proj.math.numbers.unsigned.ULong;

import java.util.Random;
import java.util.function.DoubleUnaryOperator;

public class RandUtils {
    final public static Random RANDOM = new Random();

    public static double nextDouble (double min, double max) {
        return RANDOM.nextDouble(min, max);
    }
}

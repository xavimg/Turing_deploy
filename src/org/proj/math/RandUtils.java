package org.proj.math;

import java.util.Random;

public class RandUtils {
    final public static Random RANDOM = new Random();

    public static double nextDouble (double min, double max) {
        return (max - min) * RANDOM.nextDouble() + min;
    }

    public static double nextGaussian (Random random, double mean, double std) {
        return std * random.nextGaussian() + mean;
    }
}

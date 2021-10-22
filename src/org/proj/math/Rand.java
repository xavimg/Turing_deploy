package org.proj.math;

import java.util.Random;

public class Rand {
    final private static Random rng = new Random();

    public static double nextDouble (double min, double max) {
        return rng.nextDouble(min, max);
    }
}

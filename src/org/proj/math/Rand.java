package org.proj.math;

import org.proj.math.numbers.Chungus;
import org.proj.math.numbers.unsigned.ULong;

import java.util.Random;

public class Rand {
    final public static Random RANDOM = new Random();

    public static long nextLong () {
        return RANDOM.nextLong();
    }

    public static long nextLong (long min, long max) {
        return RANDOM.nextLong(min, max);
    }

    public static ULong nextULong () {
        return ULong.ofBits(RANDOM.nextLong());
    }

    public static ULong nextULong (ULong min, ULong max) {
        return nextULong().mul(max.subtr(min)).add(min);
    }

    public static Chungus nextChungus () {
        ULong alpha = nextULong();
        ULong beta = nextULong();

        return new Chungus(alpha, beta);
    }

    public static double nextDouble (double min, double max) {
        return RANDOM.nextDouble(min, max);
    }
}

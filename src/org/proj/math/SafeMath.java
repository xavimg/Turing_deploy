package org.proj.math;

public class SafeMath {
    public static int shiftLeft (int value, int n) {
        if (n < 0) {
            return shiftRight(value, -n);
        } else if (n >= 32) {
            return 0;
        }

        return value << n;
    }

    public static int shiftRight (int value, int n) {
        if (n < 0) {
            return shiftLeft(value, -n);
        } else if (n >= 32) {
            return 0;
        }

        return value >>> n;
    }

    public static long shiftLeft (long value, int n) {
        if (n < 0) {
            return shiftRight(value, -n);
        } else if (n >= 64) {
            return 0;
        }

        return value << n;
    }

    public static long shiftRight (long value, int n) {
        if (n < 0) {
            return shiftLeft(value, -n);
        } else if (n >= 64) {
            return 0;
        }

        return value >>> n;
    }
}

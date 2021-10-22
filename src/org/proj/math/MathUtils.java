package org.proj.math;

public class MathUtils {
    final public static double PI_2 = 2 * Math.PI;
    final public static double PI2 = Math.PI * Math.PI;

    public static String toHumanString (double radians) {
        int revs = (int) (radians / PI_2);
        double angle = radians - (PI_2 * revs);

        return Double.toString(Math.toDegrees(angle));
    }

    public static double sec (double value) {
        return 1d / Math.cos(value);
    }
}

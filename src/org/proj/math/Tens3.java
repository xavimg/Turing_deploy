package org.proj.math;

import org.proj.math.matrix.Mat3;
import org.proj.math.vector.Vec3;

import java.util.function.BiFunction;
import java.util.function.IntFunction;

public class Tens3 {
    final public Mat3 x, y, z;

    public Tens3 (Mat3 x, Mat3 y, Mat3 z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public static Tens3 of (IntFunction<Mat3> function) {
        return new Tens3(function.apply(0), function.apply(1), function.apply(2));
    }

    public static Tens3 of (BiFunction<Integer, Integer, Vec3> function) {
        return of(i -> Mat3.of(j -> function.apply(i,j)));
    }

    public static Tens3 of (TriIntToDoubleFunction function) {
        return of((i,j) -> Vec3.of(k -> function.apply(i,j,k)));
    }

    public Mat3 get (int pos) {
        return switch (pos) {
            case 0 -> x;
            case 1 -> y;
            case 2 -> z;
            default -> throw new IndexOutOfBoundsException();
        };
    }

    public Vec3 get (int i, int j) {
        return get(i).get(j);
    }

    public double get (int i, int j, int k) {
        return get(i,j).get(k);
    }
}

package org.proj.math.matrix;

import kotlin.Lazy;
import org.proj.math.vector.Vec2;
import org.proj.utils.JavaLazy;

import java.util.function.IntFunction;
import java.util.function.ToDoubleBiFunction;

public class Mat2 {
    final public static Mat2 ZERO = new Mat2(Vec2.ZER0, Vec2.ZER0);

    final public Vec2 x, y;
    final private Lazy<Boolean> diagonal;

    public Mat2(Vec2 x, Vec2 y) {
        this.x = x;
        this.y = y;

        this.diagonal = new JavaLazy<>(() -> this.x.y == 0 && this.y.x == 0);
    }

    public Mat2(double x, double y) {
        this.x = new Vec2(x, 0);
        this.y = new Vec2(0, y);

        this.diagonal = new JavaLazy<>(true);
    }

    public Mat2(double xx, double xy, double yx, double yy) {
        this(new Vec2(xx, xy), new Vec2(yx, yy));
    }

    public static Mat2 of (IntFunction<Vec2> function) {
        return new Mat2(function.apply(0), function.apply(1));
    }

    public static Mat2 of (ToDoubleBiFunction<Integer, Integer> function) {
        return of(i -> Vec2.of(j -> function.applyAsDouble(i,j)));
    }

    public Vec2 get (int pos) {
        return switch (pos) {
            case 0 -> x;
            case 1 -> y;
            default -> throw new IndexOutOfBoundsException();
        };
    }

    public double get (int i, int j) {
        return get(i).get(j);
    }

    public Mat2 add (Mat2 other) {
        return new Mat2(this.x.add(other.x), this.y.add(other.y));
    }

    public Mat2 add (double other) {
        return new Mat2(this.x.add(other), this.y.add(other));
    }

    public Mat2 subtr (Mat2 other) {
        return new Mat2(this.x.add(other.x), this.y.add(other.y));
    }

    public Mat2 subtr (double other) {
        return new Mat2(this.x.add(other), this.y.add(other));
    }

    public Mat2 mul (Mat2 other) {
        return new Mat2(
                this.x.x * other.x.x + this.x.y * other.y.x,
                this.y.x * other.x.x + this.y.y * other.y.x,

                this.x.x * other.x.y + this.x.y * other.y.y,
                this.y.x * other.x.y + this.y.y * other.y.y
        );
    }

    public Vec2 mul (Vec2 other) {
        return new Vec2(
                this.x.x * other.x + this.x.y * other.y,
                this.y.x * other.x + this.y.y * other.y
        );
    }

    public Mat2 mul (double other) {
        return new Mat2(this.x.mul(other), this.y.mul(other));
    }

    public Mat2 div (double other) {
        return new Mat2(this.x.div(other), this.y.div(other));
    }

    public boolean isDiagonal () {
        return this.diagonal.getValue();
    }

    public double det () {
        return x.x * y.y - x.y * y.x;
    }

    public Mat2 inverse () {
        if (isDiagonal()) {
            return new Mat2(
                    1 / this.x.x, 0,
                    0,1 / this.y.y
            );
        }

        double det = det();
        return new Mat2(
                y.y / det, -x.y / det,
                -y.x / det, x.x / det
        );
    }
}

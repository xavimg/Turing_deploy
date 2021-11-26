package org.proj.math.matrix;

import org.proj.math.vector.Vec3;
import org.proj.utils.Lazy;

import java.util.function.IntFunction;
import java.util.function.ToDoubleBiFunction;

public class Mat3 {
    final public static Mat3 ZERO = new Mat3(Vec3.ZER0, Vec3.ZER0, Vec3.ZER0);

    final public Vec3 x, y, z;
    final private Lazy<Boolean> diagonal;

    public Mat3 (Vec3 x, Vec3 y, Vec3 z) {
        this.x = x;
        this.y = y;
        this.z = z;

        this.diagonal = new Lazy<>(() -> this.x.y == 0 & this.x.z == 0 & this.y.x == 0 & this.y.z == 0 & this.z.x == 0 & this.z.y == 0);
    }

    public Mat3 (double x, double y, double z) {
        this.x = new Vec3(x, 0, 0);
        this.y = new Vec3(0, y, 0);
        this.z = new Vec3(0, 0, z);

        this.diagonal = new Lazy<>(true);
    }

    public Mat3 (double xx, double xy, double xz, double yx, double yy, double yz, double zx, double zy, double zz) {
        this(new Vec3(xx, xy, xz), new Vec3(yx, yy, yz), new Vec3(zx, zy, zz));
    }

    public static Mat3 of (IntFunction<Vec3> function) {
        return new Mat3(function.apply(0), function.apply(1), function.apply(2));
    }

    public static Mat3 of (ToDoubleBiFunction<Integer, Integer> function) {
        return of(i -> Vec3.of(j -> function.applyAsDouble(i,j)));
    }

    public Vec3 get (int pos) {
        return switch (pos) {
            case 0 -> x;
            case 1 -> y;
            case 2 -> z;
            default -> throw new IndexOutOfBoundsException();
        };
    }

    public double get (int i, int j) {
        return get(i).get(j);
    }

    public Mat3 add (Mat3 other) {
        return new Mat3(this.x.add(other.x), this.y.add(other.y), this.z.add(other.z));
    }

    public Mat3 add (double other) {
        return new Mat3(this.x.add(other), this.y.add(other), this.z.add(other));
    }

    public Mat3 subtr (Mat3 other) {
        return new Mat3(this.x.add(other.x), this.y.add(other.y), this.z.add(other.z));
    }

    public Mat3 subtr (double other) {
        return new Mat3(this.x.add(other), this.y.add(other), this.z.add(other));
    }

    public Mat3 mul (Mat3 other) {
        return new Mat3(
                this.x.x * other.x.x + this.x.y * other.y.x + this.x.z * other.z.x,
                this.y.x * other.x.x + this.y.y * other.y.x + this.y.z * other.z.x,
                this.z.x * other.x.x + this.z.y * other.y.x + this.z.z * other.z.x,

                this.x.x * other.x.y + this.x.y * other.y.y + this.x.z * other.z.y,
                this.y.x * other.x.y + this.y.y * other.y.y + this.y.z * other.z.y,
                this.z.x * other.x.y + this.z.y * other.y.y + this.z.z * other.z.y,

                this.x.x * other.x.z + this.x.y * other.y.z + this.x.z * other.z.z,
                this.y.x * other.x.z + this.y.y * other.y.z + this.y.z * other.z.z,
                this.z.x * other.x.z + this.z.y * other.y.z + this.z.z * other.z.z
        );
    }

    public Vec3 mul (Vec3 other) {
        return new Vec3(
                this.x.x * other.x + this.x.y * other.y + this.x.z * other.z,
                this.y.x * other.x + this.y.y * other.y + this.y.z * other.z,
                this.z.x * other.x + this.z.y * other.y + this.z.z * other.z
        );
    }

    public Mat3 mul (double other) {
        return new Mat3(this.x.mul(other), this.y.mul(other), this.z.mul(other));
    }

    public Mat3 div (double other) {
        return new Mat3(this.x.div(other), this.y.div(other), this.z.div(other));
    }

    public boolean isDiagonal () {
        return this.diagonal.get();
    }

    public Mat3 inverse () {
        if (isDiagonal()) {
            return new Mat3(
                    1 / this.x.x,0,0,
                    0,1 / this.y.y,0,
                    0,0,1 / this.z.z
            );
        }

        double det1 = this.x.x * (this.y.y * this.z.z - this.y.z * this.z.y);
        double det2 = -this.x.y * (this.y.x * this.z.z - this.x.z * this.z.x);
        double det3 = this.x.z * (this.y.x * this.z.y - this.y.y * this.z.x);
        double det = det1 + det2 + det3;

        return new Mat3(
                det1 / det, (this.x.z * this.z.y - this.x.y * this.z.z) / det, (this.x.y * this.y.z - this.x.z * this.y.y) / det,
                det2 / det, (this.x.x * this.z.z - this.x.z * this.z.x) / det, (this.x.z * this.y.x - this.x.x * this.y.z) / det,
                det3 / det, (this.x.y * this.z.x - this.x.x * this.z.y) / det, (this.x.x * this.y.y - this.x.y * this.y.x) / det
        );
    }
}

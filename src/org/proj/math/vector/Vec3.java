package org.proj.math.vector;

import java.util.function.IntToDoubleFunction;

final public class Vec3 {
    final public static Vec3 ZER0 = new Vec3(0, 0, 0);
    final public double x, y, z;

    public Vec3 (double x, double y, double z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public Vec3 (double x, Vec2 yz) {
        this(x, yz.x, yz.y);
    }

    public Vec3 (Vec2 xy, double z) {
        this(xy.x, xy.y, z);
    }

    public double get (int pos) {
        return switch (pos) {
            case 0 -> x;
            case 1 -> y;
            case 2 -> z;
            default -> throw new IndexOutOfBoundsException();
        };
    }

    public static Vec3 of (IntToDoubleFunction function) {
        return new Vec3(function.applyAsDouble(0), function.applyAsDouble(1), function.applyAsDouble(2));
    }

    public Vec3 add (Vec3 other) {
        return new Vec3(this.x + other.x, this.y + other.y, this.z + other.z);
    }

    public Vec3 add (double other) {
        return new Vec3(this.x + other, this.y + other, this.z + other);
    }

    public Vec3 subtr (Vec3 other) {
        return new Vec3(this.x - other.x, this.y - other.y, this.z - other.z);
    }

    public Vec3 subtr (double other) {
        return new Vec3(this.x - other, this.y - other, this.z - other);
    }

    public Vec3 mul (Vec3 other) {
        return new Vec3(this.x * other.x, this.y * other.y, this.z * other.z);
    }

    public Vec3 mul (double other) {
        return new Vec3(this.x * other, this.y * other, this.z * other);
    }

    public Vec3 div (Vec3 other) {
        return new Vec3(this.x / other.x, this.y / other.y, this.z / other.z);
    }

    public Vec3 div (double other) {
        return new Vec3(this.x / other, this.y / other, this.z / other);
    }

    public Vec3 invDiv (double other) {
        return new Vec3(other / this.x, other / this.y, other / this.z);
    }

    public double sum () {
        return this.x + this.y + this.z;
    }

    public double dot (Vec3 other) {
        return this.x * other.x + this.y * other.y + this.z * other.z;
    }

    public double length2 () {
        return this.x * this.x + this.y * this.y + this.z * this.z;
    }

    public double length () {
        return Math.sqrt(this.length2());
    }

    public Vec3 unit () {
        return this.div(this.length());
    }
}

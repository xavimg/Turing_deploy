package org.proj.math.vector;

import java.util.function.IntToDoubleFunction;

final public class Vec2 {
    final public static Vec2 ZER0 = new Vec2(0, 0);

    final public double x, y;

    public Vec2 (double x, double y) {
        this.x = x;
        this.y = y;
    }

    public static Vec2 of (IntToDoubleFunction function) {
        return new Vec2(function.applyAsDouble(0), function.applyAsDouble(1));
    }

    public double get (int pos) {
        return switch (pos) {
            case 0 -> x;
            case 1 -> y;
            default -> throw new IndexOutOfBoundsException();
        };
    }

    public Vec2 add (Vec2 other) {
        return new Vec2(this.x + other.x, this.y + other.y);
    }

    public Vec2 add (double other) {
        return new Vec2(this.x + other, this.y + other);
    }

    public Vec2 subtr (Vec2 other) {
        return new Vec2(this.x - other.x, this.y - other.y);
    }

    public Vec2 subtr (double other) {
        return new Vec2(this.x - other, this.y - other);
    }

    public Vec2 mul (Vec2 other) {
        return new Vec2(this.x * other.x, this.y * other.y);
    }

    public Vec2 mul (double other) {
        return new Vec2(this.x * other, this.y * other);
    }

    public Vec2 div (Vec2 other) {
        return new Vec2(this.x / other.x, this.y / other.y);
    }

    public Vec2 div (double other) {
        return new Vec2(this.x / other, this.y / other);
    }

    public Vec2 invDiv (double other) {
        return new Vec2(other / this.x, other / this.y);
    }

    public double sum () {
        return this.x + this.y;
    }

    public double dot (Vec2 other) {
        return this.x * other.x + this.y * other.y;
    }

    public double length2 () {
        return this.x * this.x + this.y * this.y;
    }

    public double length () {
        return Math.hypot(this.x, this.y);
    }

    public Vec2 unit () {
        return this.div(this.length());
    }
}

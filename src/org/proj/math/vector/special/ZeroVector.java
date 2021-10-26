package org.proj.math.vector.special;

import org.proj.math.vector.LazyVector;
import org.proj.math.vector.Vector;

import java.math.BigDecimal;

public class ZeroVector extends Vector {
    public ZeroVector(int size) {
        super(size);
    }

    @Override
    final public double get(int i) {
        return 0;
    }

    @Override
    public Vector add (Vector other) {
        return other;
    }

    @Override
    public ConstantVector add (double other) {
        return new ConstantVector(size, other);
    }

    @Override
    public Vector subtr (Vector other) {
        return other.mul(-1);
    }

    @Override
    public ConstantVector subtr (double other) {
        return new ConstantVector(size, -other);
    }

    @Override
    public ZeroVector mul (Vector other) {
        return this;
    }

    @Override
    public Vector mul (double other) {
        return this;
    }

    @Override
    public Vector div (Vector other) {
        return this;
    }

    @Override
    public Vector div (double other) {
        return this;
    }

    @Override
    public Vector invDiv (double other) {
        throw new ArithmeticException("Division by zero");
    }

    @Override
    public double sum() {
        return 0;
    }

    @Override
    public double dot (Vector other) {
        return 0;
    }

    @Override
    public double length2() {
        return 0;
    }

    @Override
    public double length() {
        return 0;
    }
}

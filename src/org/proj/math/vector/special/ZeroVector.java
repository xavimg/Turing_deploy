package org.proj.math.vector.special;

import org.proj.math.vector.LazyVector;
import org.proj.math.vector.Vector;

import java.math.BigDecimal;

public class ZeroVector extends Vector {
    public ZeroVector(int size) {
        super(size);
    }

    @Override
    final public BigDecimal get(int i) {
        return BigDecimal.ZERO;
    }

    @Override
    public Vector add (Vector other) {
        return other;
    }

    @Override
    public ConstantVector add (BigDecimal other) {
        return new ConstantVector(size, other);
    }

    @Override
    public Vector subtr (Vector other) {
        return other.mul(BigDecimal.ONE.negate());
    }

    @Override
    public ConstantVector subtr (BigDecimal other) {
        return new ConstantVector(size, other.negate());
    }

    @Override
    public ZeroVector mul (Vector other) {
        return this;
    }

    @Override
    public Vector mul (BigDecimal other) {
        return this;
    }

    @Override
    public Vector div (Vector other) {
        return this;
    }

    @Override
    public Vector div (BigDecimal other) {
        return this;
    }

    @Override
    public Vector invDiv (BigDecimal other) {
        throw new ArithmeticException("Division by zero");
    }

    @Override
    public BigDecimal sum() {
        return BigDecimal.ZERO;
    }

    @Override
    public BigDecimal dot (Vector other) {
        return BigDecimal.ZERO;
    }

    @Override
    public BigDecimal length2() {
        return BigDecimal.ZERO;
    }

    @Override
    public BigDecimal length() {
        return BigDecimal.ZERO;
    }
}

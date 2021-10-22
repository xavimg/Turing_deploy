package org.proj.math.matrix.special;

import org.proj.math.matrix.Matrix;
import org.proj.math.vector.special.ZeroVector;

import java.math.BigDecimal;

public class ZeroMatrix extends Matrix {
    public ZeroMatrix (int rows, int cols) {
        super(rows, cols);
    }

    @Override
    public BigDecimal get(int i, int j) {
        return BigDecimal.ZERO;
    }

    @Override
    public ZeroVector get (int i) {
        return new ZeroVector(cols);
    }

    @Override
    public Matrix add (Matrix other) {
        return other;
    }

    @Override
    public Matrix subtr (Matrix other) {
        return other.mul(BigDecimal.ONE.negate());
    }

    @Override
    public ZeroMatrix mul (Matrix other) {
        return this;
    }

    @Override
    public ZeroMatrix mul (BigDecimal other) {
        return this;
    }

    @Override
    public ZeroMatrix div (BigDecimal other) {
        return this;
    }

    @Override
    public ZeroMatrix transp() {
        return this;
    }

    @Override
    public Matrix inverse() {
        throw new ArithmeticException("Division by zero");
    }

    @Override
    public Matrix rref() {
        throw new ArithmeticException("Division by zero");
    }
}

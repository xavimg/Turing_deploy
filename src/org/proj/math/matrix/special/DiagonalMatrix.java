package org.proj.math.matrix.special;

import org.proj.math.matrix.Matrix;
import org.proj.math.vector.Vector;

import java.math.BigDecimal;

public class DiagonalMatrix extends Matrix {
    final private Vector values;

    public DiagonalMatrix(Vector values) {
        super(values.size, values.size);
        this.values = values;
    }

    public DiagonalMatrix(double... values) {
        this(Vector.of(values));
    }

    @Override
    final public double get (int i, int j) {
        return i == j ? values.get(i) : 0;
    }

    final public Vector getVector () {
        return values;
    }

    // ARITHMETIC
    public DiagonalMatrix add (DiagonalMatrix other) {
        return new DiagonalMatrix(values.add(other.values));
    }

    public DiagonalMatrix subtr (DiagonalMatrix other) {
        return new DiagonalMatrix(values.subtr(other.values));
    }

    public DiagonalMatrix mul (DiagonalMatrix other) {
        return new DiagonalMatrix(values.mul(other.values));
    }

    @Override
    public Vector mul (Vector other) {
        return values.mul(other);
    }

    @Override
    public DiagonalMatrix mul (double other) {
        return new DiagonalMatrix(values.mul(other));
    }

    @Override
    public DiagonalMatrix div (double other) {
        return new DiagonalMatrix(values.div(other));
    }

    @Override
    public DiagonalMatrix inverse() {
        return new DiagonalMatrix(values.invDiv(1));
    }

    @Override
    public DiagonalMatrix transp () {
        return this;
    }
}

package org.proj.math.matrix;

import org.proj.math.vector.LazyVector;
import org.proj.math.vector.Vector;

import java.math.BigDecimal;

public abstract class LazyMatrix extends Matrix.OfVector {
    final private Vector[] values;

    public LazyMatrix (int rows, int cols) {
        super(rows, cols);
        this.values = new LazyVector[rows];
    }

    private LazyMatrix (int cols, LazyVector... rows) {
        super(rows.length, cols);
        this.values = rows;
    }

    public abstract double compute (int i, int j);

    public Vector compute (int i) {
        return new LazyVector (cols) {
            @Override
            public double compute (int j) {
                return LazyMatrix.this.compute(i, j);
            }
        };
    }

    @Override
    public Vector get (int i) {
        Vector value = values[i];
        if (value == null) {
            value = values[i] = compute(i);
        }

        return value;
    }

    // STATIC
    public static LazyMatrix of (int cols, LazyVector... rows) {
        return new LazyMatrix (rows.length, cols) {
            @Override
            public double compute(int i, int j) {
                throw new AssertionError();
            }
        };
    }

    // SUBCLASSES
    public abstract static class OfVector extends LazyMatrix {
        public OfVector (int rows, int cols) {
            super(rows, cols);
        }

        @Override
        public abstract Vector compute (int i);

        @Override
        public double compute (int i, int j) {
            return compute(i).get(j);
        }
    }
}

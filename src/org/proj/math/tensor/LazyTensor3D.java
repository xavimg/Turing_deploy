package org.proj.math.tensor;

import org.proj.math.matrix.LazyMatrix;
import org.proj.math.matrix.Matrix;
import org.proj.math.vector.Vector;

public abstract class LazyTensor3D extends Tensor3D {
    final private Matrix[] values;

    public LazyTensor3D (int alpha, int beta, int gamma) {
        super(alpha, beta, gamma);
        this.values = new Matrix[alpha];
    }

    public abstract double compute (int i, int j, int k);

    public Matrix compute (int i) {
        return new LazyMatrix (beta, gamma) {
            @Override
            public double compute (int j, int k) {
                return LazyTensor3D.this.compute(i, j, k);
            }
        };
    }

    @Override
    public Matrix get (int i) {
        Matrix value = values[i];
        if (value == null) {
            value = values[i] = compute(i);
        }

        return value;
    }

    @Override
    public Vector get (int i, int j) {
        return get(i).get(j);
    }

    @Override
    public double get (int x, int y, int z) {
        return get(x).get(y, z);
    }

    // SUBCLASSES
    public abstract static class OfMatrix extends LazyTensor3D {
        public OfMatrix (int alpha, int beta, int gamma) {
            super(alpha, beta, gamma);
        }

        @Override
        public abstract Matrix compute (int i);

        @Override
        public double compute (int x, int y, int z) {
            return compute(x).get(y, z);
        }
    }
}

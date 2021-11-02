package org.proj.math.tensor;

import org.proj.math.matrix.Matrix;
import org.proj.math.vector.Vector;

import java.math.BigDecimal;
import java.util.Iterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public abstract class Tensor3D implements Iterable<Matrix> {
    final public int alpha, beta, gamma;

    public Tensor3D (int alpha, int beta, int gamma) {
        this.alpha = alpha;
        this.beta = beta;
        this.gamma = gamma;
    }

    public abstract double get (int x, int y, int z);

    public Matrix get (int i) {
        return new Matrix (beta, gamma) {
            @Override
            public double get (int j, int k) {
                return Tensor3D.this.get(i, j, k);
            }
        };
    }

    public Vector get (int i, int j) {
        return new Vector (gamma) {
            @Override
            public double get(int k) {
                return Tensor3D.this.get(i, j, k);
            }
        };
    }

    @Override
    public Iterator<Matrix> iterator() {
        return new Iterator<Matrix>() {
            int i = 0;

            @Override
            public boolean hasNext() {
                return i < alpha;
            }

            @Override
            public Matrix next() {
                return get(i++);
            }
        };
    }

    @Override
    public Spliterator<Matrix> spliterator() {
        return Spliterators.spliterator(iterator(), alpha, 0);
    }

    public Stream<Matrix> stream () {
        return StreamSupport.stream(spliterator(), false);
    }

    public Stream<Matrix> parallelStream () {
        return StreamSupport.stream(spliterator(), true);
    }

    @Override
    public String toString () {
        StringBuilder builder = new StringBuilder();
        for (Matrix val: this) {
            builder.append(", ").append(val);
        }

        return "["+builder.substring(2)+"]";
    }

    public OfArray toStatic () {
        return new OfArray(parallelStream().map(Matrix::toStatic).toArray(Matrix[]::new));
    }

    // SUBCLASSES
    public static class OfArray extends Tensor3D {
        final private Matrix[] values;

        public OfArray (Matrix... values) {
            super(values.length, values[0].rows, values[0].cols);
            this.values = values;
        }

        @Override
        public double get (int x, int y, int z) {
            return values[x].get(y, z);
        }

        @Override
        public OfArray toStatic() {
            return this;
        }
    }

    public abstract static class OfVector extends Tensor3D {
        public OfVector (int alpha, int beta, int gamma) {
            super(alpha, beta, gamma);
        }

        @Override
        public abstract Vector get (int i, int j);

        @Override
        public double get (int x, int y, int z) {
            return get(x, y).get(z);
        }
    }

    public abstract static class OfMatrix extends Tensor3D {
        public OfMatrix (int alpha, int beta, int gamma) {
            super(alpha, beta, gamma);
        }

        @Override
        public abstract Matrix get (int i);

        @Override
        public double get (int x, int y, int z) {
            return get(x).get(y, z);
        }
    }
}

package org.proj.math.matrix;

import org.proj.math.DifferentSizedException;
import org.proj.math.vector.Vector;

import java.math.BigDecimal;
import java.util.Iterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public abstract class Matrix implements Iterable<Vector> {
    final public int rows, cols;

    public Matrix (int rows, int cols) {
        this.rows = rows;
        this.cols = cols;
    }

    public abstract BigDecimal get (int i, int j);

    public Vector get (int i) {
        return new Vector (cols) {
            @Override
            public BigDecimal get (int j) {
                return Matrix.this.get(i, j);
            }
        };
    }

    // ARITHMETIC
    public Matrix add (Matrix other) {
        assertSameSize(other);
        return new LazyMatrix.OfVector (rows, cols) {
            @Override
            public Vector compute(int i) {
                return Matrix.this.get(i).add(other.get(i));
            }
        };
    }

    public Matrix subtr (Matrix other) {
        assertSameSize(other);
        return new LazyMatrix.OfVector (rows, cols) {
            @Override
            public Vector compute(int i) {
                return Matrix.this.get(i).subtr(other.get(i));
            }
        };
    }

    public Matrix mul (Matrix other) {
        if (cols != other.rows) {
            throw new DifferentSizedException();
        }

        return new LazyMatrix (rows, other.cols) {
            final Matrix transp = other.transp();

            @Override
            public BigDecimal compute (int i, int j) {
                return Matrix.this.get(i).dot(transp.get(j));
            }
        };
    }

    public Matrix mul (BigDecimal other) {
        return new LazyMatrix.OfVector (rows, cols) {
            @Override
            public Vector compute(int i) {
                return Matrix.this.get(i).mul(other);
            }
        };
    }

    public Matrix div (BigDecimal other) {
        return new LazyMatrix.OfVector (rows, cols) {
            @Override
            public Vector compute(int i) {
                return Matrix.this.get(i).div(other);
            }
        };
    }

    // METHODS
    public Matrix transp () {
        return new Matrix (cols, rows) {
            @Override
            public BigDecimal get (int i, int j) {
                return Matrix.this.get(j, i);
            }
        };
    }

    public Matrix inverse () {
        Matrix alpha = new Matrix(rows, 2 * cols) {
            @Override
            public BigDecimal get (int i, int j) {
                if (j >= Matrix.this.cols) {
                    return i == (j - Matrix.this.cols) ? BigDecimal.ONE : BigDecimal.ZERO;
                }

                return Matrix.this.get(i, j);
            }
        }.rref();

        return new Matrix (rows, cols) {
            @Override
            public BigDecimal get (int i, int j) {
                return alpha.get(i, j + cols);
            }
        };
    }

    public Matrix rref () {
        Vector[] rws = stream().toArray(Vector[]::new);

        for (int i=0;i<rows;i++) {
            Vector row = rws[i];
            rws[i] = row = row.div(row.get(i));

            for (int j=0;j<rows;j++) {
                if (i == j) {
                    continue;
                }

                Vector row2 = rws[j];
                rws[j] = row2.subtr(row.mul(row2.get(i)));
            }
        }

        return Matrix.of(rws);
    }

    // OTHERS
    private void assertSameSize (Matrix other) throws DifferentSizedException {
        if (rows != other.rows || cols != other.cols) {
            throw new DifferentSizedException();
        }
    }

    @Override
    public String toString () {
        StringBuilder builder = new StringBuilder();
        for (Vector val: this) {
            builder.append(", ").append(val);
        }

        return "["+builder.substring(2)+"]";
    }

    // STREAMS
    @Override
    public Iterator<Vector> iterator() {
        return new Iterator<>() {
            int i = 0;

            @Override
            public boolean hasNext() {
                return i < rows;
            }

            @Override
            public Vector next() {
                return get(i++);
            }
        };
    }

    @Override
    public Spliterator<Vector> spliterator() {
        return Spliterators.spliterator(iterator(), rows, 0);
    }

    public Stream<Vector> stream () {
        return StreamSupport.stream(spliterator(), false);
    }

    public Stream<Vector> parallelStream () {
        return StreamSupport.stream(spliterator(), true);
    }

    // STATIC
    public static Matrix of (BigDecimal[]... array) {
        return new Matrix (array.length, array[0].length) {
            @Override
            public BigDecimal get(int i, int j) {
                return array[i][j];
            }
        };
    }

    public static OfArray of (Vector... array) {
        return new OfArray(array);
    }

    // SUBCLASSES
    public abstract static class OfVector extends Matrix {
        public OfVector (int rows, int cols) {
            super(rows, cols);
        }

        @Override
        public abstract Vector get (int i);

        @Override
        public BigDecimal get (int i, int j) {
            return OfVector.this.get(i).get(j);
        }
    }

    public static class OfArray extends OfVector {
        final private Vector[] values;

        public OfArray (Vector... values) {
            super(values.length, values[0].size);
            this.values = values;
        }

        @Override
        public Vector get(int i) {
            return values[i];
        }
    }
}

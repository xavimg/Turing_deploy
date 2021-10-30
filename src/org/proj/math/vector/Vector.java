package org.proj.math.vector;

import org.proj.math.DifferentSizedException;

import java.math.BigDecimal;
import java.math.MathContext;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.DoubleStream;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public abstract class Vector implements Iterable<Double> {
    final public int size;

    public Vector (int size) {
        this.size = size;
    }

    public abstract double get (int i);

    // ARITHMETIC
    public Vector add (Vector other) {
        assertSameSize(other);
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Vector.this.get(pos) + other.get(pos);
            }
        };
    }

    public Vector add (double other) {
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Vector.this.get(pos) + other;
            }
        };
    }

    public Vector subtr (Vector other) {
        assertSameSize(other);
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Vector.this.get(pos) - other.get(pos);
            }
        };
    }

    public Vector subtr (double other) {
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Vector.this.get(pos) - other;
            }
        };
    }

    public Vector mul (Vector other) {
        assertSameSize(other);
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Vector.this.get(pos) * other.get(pos);
            }
        };
    }

    public Vector mul (double other) {
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Vector.this.get(pos) * other;
            }
        };
    }

    public Vector div (Vector other) {
        assertSameSize(other);
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Vector.this.get(pos) / other.get(pos);
            }
        };
    }

    public Vector div (double other) {
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Vector.this.get(pos) / other;
            }
        };
    }

    public Vector invDiv (double other) {
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return other / Vector.this.get(pos);
            }
        };
    }

    public Vector abs () {
        return new LazyVector (size) {
            @Override
            public double compute (int pos) {
                return Math.abs(Vector.this.get(pos));
            }
        };
    }

    // METHODS
    public double sum () {
        return parallelStream().sum();
    }

    public double dot (Vector other) {
        return mul(other).sum();
    }

    public double length2 () {
        return dot(this);
    }

    public double length () {
        return Math.sqrt(length2());
    }

    public Vector unit () {
        return div(length());
    }

    public Vector.OfArray toStatic () {
        return new OfArray(parallelStream().toArray());
    }

    // SLICING
    public Vector copyOf (int offset, int stride, int size) {
        return new Vector (size) {
            @Override
            public double get (int i) {
                return Vector.this.get(offset + i * stride);
            }
        };
    }

    public Vector copyOf (int offset, int size) {
        return new Vector (size) {
            @Override
            public double get (int i) {
                return Vector.this.get(offset + i);
            }
        };
    }

    public Vector copyOf (int offset) {
        return copyOf(offset, size - offset);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Vector doubles = (Vector) o;

        if (size != doubles.size) {
            return false;
        }

        for (int i=0;i<size;i++) {
            if (get(i) != doubles.get(i)) {
                return false;
            }
        }

        return true;
    }

    @Override
    public int hashCode() {
        return Objects.hash(size);
    }

    // STREAMS
    @Override
    public PrimitiveIterator.OfDouble iterator() {
        return new PrimitiveIterator.OfDouble() {
            int i = 0;

            @Override
            public double nextDouble() {
                return get(i++);
            }

            @Override
            public boolean hasNext() {
                return i < size;
            }
        };
    }

    @Override
    public Spliterator.OfDouble spliterator() {
        return Spliterators.spliterator(iterator(), size, 0);
    }

    public DoubleStream stream () {
        return StreamSupport.doubleStream(spliterator(), false);
    }

    public DoubleStream parallelStream () {
        return StreamSupport.doubleStream(spliterator(), true);
    }

    // OTHERS
    private void assertSameSize (Vector other) throws DifferentSizedException {
        if (size != other.size) {
            throw new DifferentSizedException();
        }
    }

    public double[] toArray () {
        return parallelStream().toArray();
    }

    @Override
    public String toString () {
        return "[" + parallelStream().mapToObj(Double::toString).collect(Collectors.joining(", ")) + "]";
    }

    // STATIC
    public static OfArray of (double... array) {
        return new OfArray(array);
    }

    // SUBCLASSES
    public static class OfArray extends Vector {
        final private double[] array;

        public OfArray (double... array) {
            super(array.length);
            this.array = array;
        }

        @Override
        public double get (int i) {
            return array[i];
        }

        @Override
        public double[] toArray() {
            return array.clone();
        }

        @Override
        public OfArray toStatic() {
            return this;
        }
    }
}

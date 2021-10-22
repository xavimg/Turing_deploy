package org.proj.math.vector;

import org.proj.math.DifferentSizedException;

import java.math.BigDecimal;
import java.math.MathContext;
import java.util.*;
import java.util.stream.DoubleStream;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public abstract class Vector implements Iterable<BigDecimal> {
    final public int size;

    public Vector (int size) {
        this.size = size;
    }

    public abstract BigDecimal get (int i);

    // ARITHMETIC
    public Vector add (Vector other) {
        assertSameSize(other);
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return Vector.this.get(pos).add(other.get(pos));
            }
        };
    }

    public Vector add (BigDecimal other) {
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return Vector.this.get(pos).add(other);
            }
        };
    }

    public Vector subtr (Vector other) {
        assertSameSize(other);
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return Vector.this.get(pos).subtract(other.get(pos));
            }
        };
    }

    public Vector subtr (BigDecimal other) {
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return Vector.this.get(pos).subtract(other);
            }
        };
    }

    public Vector mul (Vector other) {
        assertSameSize(other);
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return Vector.this.get(pos).multiply(other.get(pos));
            }
        };
    }

    public Vector mul (BigDecimal other) {
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return Vector.this.get(pos).multiply(other);
            }
        };
    }

    public Vector div (Vector other) {
        assertSameSize(other);
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return Vector.this.get(pos).divide(other.get(pos), MathContext.DECIMAL128);
            }
        };
    }

    public Vector div (BigDecimal other) {
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return Vector.this.get(pos).divide(other, MathContext.DECIMAL128);
            }
        };
    }

    public Vector invDiv (BigDecimal other) {
        return new LazyVector (size) {
            @Override
            public BigDecimal compute (int pos) {
                return other.divide(Vector.this.get(pos), MathContext.DECIMAL128);
            }
        };
    }

    // METHODS
    public BigDecimal sum () {
        return parallelStream().reduce(BigDecimal::add).get();
    }

    public BigDecimal dot (Vector other) {
        return mul(other).sum();
    }

    public BigDecimal length2 () {
        return dot(this);
    }

    public BigDecimal length () {
        return length2().sqrt(MathContext.DECIMAL128);
    }

    public Vector unit () {
        return div(length());
    }

    // SLICING
    public Vector copyOf (int offset, int stride, int size) {
        return new Vector (size) {
            @Override
            public BigDecimal get (int i) {
                return Vector.this.get(offset + i * stride);
            }
        };
    }

    public Vector copyOf (int offset, int size) {
        return new Vector (size) {
            @Override
            public BigDecimal get (int i) {
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
            if (!get(i).equals(doubles.get(i))) {
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
    public Iterator<BigDecimal> iterator() {
        return new Iterator<BigDecimal>() {
            int i = 0;

            @Override
            public BigDecimal next() {
                return get(i++);
            }

            @Override
            public boolean hasNext() {
                return i < size;
            }
        };
    }

    @Override
    public Spliterator<BigDecimal> spliterator() {
        return Spliterators.spliterator(iterator(), size, 0);
    }

    public Stream<BigDecimal> stream () {
        return StreamSupport.stream(spliterator(), false);
    }

    public Stream<BigDecimal> parallelStream () {
        return StreamSupport.stream(spliterator(), true);
    }

    // OTHERS
    private void assertSameSize (Vector other) throws DifferentSizedException {
        if (size != other.size) {
            throw new DifferentSizedException();
        }
    }

    @Override
    public String toString () {
        StringBuilder builder = new StringBuilder();
        for (BigDecimal val: this) {
            builder.append(", ").append(val);
        }

        return "["+builder.substring(2)+"]";
    }

    // STATIC
    public static OfArray of (BigDecimal... array) {
        return new OfArray(array);
    }

    public static OfArray of (double... array) {
        return of(Arrays.stream(array).mapToObj(BigDecimal::valueOf).toArray(BigDecimal[]::new));
    }

    // SUBCLASSES
    public static class OfArray extends Vector {
        final private BigDecimal[] array;

        public OfArray (BigDecimal... array) {
            super(array.length);
            this.array = array;
        }

        @Override
        public BigDecimal get (int i) {
            return array[i];
        }
    }
}

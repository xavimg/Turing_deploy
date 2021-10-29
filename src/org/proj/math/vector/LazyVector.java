package org.proj.math.vector;

import java.math.BigDecimal;

public abstract class LazyVector extends Vector {
    final private Double[] values;

    public LazyVector (int size) {
        super(size);
        this.values = new Double[size];
    }

    public abstract double compute (int pos);

    @Override
    public double get (int i) {
        Double value = values[i];
        if (value == null) {
            value = values[i] = compute(i);
        }

        return value;
    }
}

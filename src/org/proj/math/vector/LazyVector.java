package org.proj.math.vector;

import java.math.BigDecimal;

public abstract class LazyVector extends Vector {
    final private BigDecimal[] values;

    public LazyVector (int size) {
        super(size);
        this.values = new BigDecimal[size];
    }

    public abstract BigDecimal compute (int pos);

    @Override
    public BigDecimal get (int i) {
        BigDecimal value = values[i];
        if (value == null) {
            value = values[i] = compute(i);
        }

        return value;
    }

    public Vector.OfArray toStatic () {
        return new Vector.OfArray(parallelStream().toArray(BigDecimal[]::new));
    }
}

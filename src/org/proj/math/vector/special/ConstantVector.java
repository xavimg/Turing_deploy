package org.proj.math.vector.special;

import org.proj.math.vector.Vector;

import java.math.BigDecimal;

public class ConstantVector extends Vector {
    final private double value;

    public ConstantVector (int size, double value) {
        super(size);
        this.value = value;
    }

    @Override
    public double get (int i) {
        return value;
    }

    @Override
    public double sum () {
        return value * size;
    }
}

package org.proj.math.vector.special;

import org.proj.math.vector.Vector;

import java.math.BigDecimal;

public class ConstantVector extends Vector {
    final private BigDecimal value;

    public ConstantVector (int size, BigDecimal value) {
        super(size);
        this.value = value;
    }

    @Override
    public BigDecimal get(int i) {
        return value;
    }

    @Override
    public BigDecimal sum () {
        return value.multiply(BigDecimal.valueOf(size));
    }
}

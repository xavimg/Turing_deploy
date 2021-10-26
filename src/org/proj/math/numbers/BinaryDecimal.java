package org.proj.math.numbers;

import org.proj.math.numbers.unsigned.UInt;
import org.proj.utils.Compare;

import java.math.BigInteger;

/**
 * Base 2 fixed point arithmetic implementation <br>
 * <ul>
 *     <li><b>Description</b>: x = magnitude * 2<sup>-scale</sup>
 * </ul>
 * @author Alex Andreba
 */
public class BinaryDecimal extends Number {
    final public BigInteger magnitude;
    final public UInt scale;

    public BinaryDecimal (BigInteger magnitude, UInt scale) {
        this.magnitude = magnitude;
        this.scale = scale;
    }

    // ARITHMETIC
    public BinaryDecimal add (BinaryDecimal other) {
        int delta = scale.subtrs(other.scale);
        if (delta >= 0) {
            return new BinaryDecimal(magnitude.add(other.magnitude.shiftLeft(delta)), scale);
        }

        return new BinaryDecimal(magnitude.shiftLeft(-delta).add(other.magnitude), other.scale);
    }

    // TO STRING
    @Override
    public String toString() {
        String integerStr = bigIntValue().toString();
        return integerStr+".???";
    }

    // NUMBER
    @Override
    public int intValue() {
        return bigIntValue().intValue();
    }

    @Override
    public long longValue() {
        return bigIntValue().longValue();
    }

    public BigInteger bigIntValue () {
        return magnitude.shiftRight(scale.intValue());
    }

    @Override
    public float floatValue() {
        return 0; // TODO
    }

    @Override
    public double doubleValue() {
        return 0; // TODO
    }

    // VALUE OF
    public static BinaryDecimal valueOf (long value) {
        return new BinaryDecimal(BigInteger.valueOf(value), UInt.ZERO);
    }

    public static BinaryDecimal valueOf (double value) {
        int exp = Math.getExponent(value);
        BigInteger mant = BigInteger.valueOf((Double.doubleToLongBits(value) & 0xfffffffffffffL) | 0x10000000000000L);

        if (exp > 0) {
            mant = mant.shiftLeft(exp);
            exp = 0;
        }

        return new BinaryDecimal(mant, UInt.valueOf(52 - exp));
    }
}

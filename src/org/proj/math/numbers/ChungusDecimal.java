package org.proj.math.numbers;

import java.math.BigInteger;

/**
 * Base 2 fixed point arithmetic implementation <br>
 * <ul>
 *     <li><b>Description</b>: x = magnitude * 2<sup>-scale</sup></li>
 * </ul>
 * @author Alex Andreba
 */
public class ChungusDecimal extends Number {
    final public static ChungusDecimal ZERO = valueOf(BigInteger.ZERO);
    final public static ChungusDecimal ONE = valueOf(BigInteger.ONE);
    final public static ChungusDecimal TEN = valueOf(BigInteger.valueOf(10));

    final public static ChungusDecimal MIN_NORMAL = new ChungusDecimal(BigInteger.ZERO, 1);

    final public BigInteger magnitude;
    final public int scale; // UNSIGNED

    public ChungusDecimal(BigInteger magnitude, int scale) {
        this.magnitude = magnitude;
        this.scale = scale;
    }

    public ChungusDecimal(long magnitude, int scale) {
        this(BigInteger.valueOf(magnitude), scale);
    }

    // OPERATIONS
    public ChungusDecimal negate () {
        return new ChungusDecimal(magnitude.negate(), scale);
    }

    // ARITHMETIC
    public ChungusDecimal add (ChungusDecimal other) {
        if (scale == other.scale) {
            return new ChungusDecimal(magnitude.add(other.magnitude), scale);
        }

        int delta = scale - other.scale;
        if (delta > 0) { // THIS > OTHER
            return new ChungusDecimal(magnitude.add(other.magnitude.shiftLeft(delta)), scale);
        }

        return new ChungusDecimal(other.magnitude.add(magnitude.shiftLeft(-delta)), other.scale);
    }

    public ChungusDecimal subtr (ChungusDecimal other) {
        return add(other.negate());
    }

    // TO STRING
    public String toBinaryString () {
        StringBuilder builder = new StringBuilder(magnitude.toString(2));
        builder.insert(builder.length() - scale, '.');

        return builder.toString();
    }

    @Override
    public String toString() {
        return null;
    }

    // VALUE OF
    public static ChungusDecimal valueOf (BigInteger value) {
        return new ChungusDecimal(value, 0);
    }

    public static ChungusDecimal valueOf (long value) {
        return new ChungusDecimal(value, 0);
    }

    public static ChungusDecimal valueOf (double value) {
        int exp = Math.getExponent(value);
        long mant = (1L << 52) | Double.doubleToLongBits(value) & 0xfffffffffffffL;

        if (exp > 52) {
            return null; // TODO
        }

        return new ChungusDecimal(mant, 52 - exp);
    }

    // IMPLEMENT NUMBER
    public BigInteger bigIntegerValue () {
        return magnitude.shiftRight(1);
    }

    @Override
    public int intValue() {
        return 0; // TODO
    }

    @Override
    public long longValue() {
        return 0; // TODO
    }

    @Override
    public float floatValue() {
        return 0; // TODO
    }

    @Override
    public double doubleValue() {
        return 0; // TODO
    }
}

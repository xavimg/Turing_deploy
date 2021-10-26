package org.proj.math.numbers;

import org.proj.math.SafeMath;

final public class ULong extends Number implements Cloneable, Comparable<ULong> {
    final public static ULong ZERO = valueOf(0);
    final public static ULong ONE = valueOf(1);
    final public static ULong TWO = valueOf(2);
    final public static ULong TEN = valueOf(10);

    final public static ULong MIN_VALUE = ZERO;
    final public static ULong MAX_VALUE = ofBits(-1);

    final private static double DOUBLE_DELTA = Math.scalb(1, 63);
    final private static float FLOAT_DELTA = Math.scalb(1, 63);

    final public long bits;

    private ULong (long bits) {
        this.bits = bits;
    }

    // BITWISE
    public ULong not () {
        return new ULong(~bits);
    }

    public ULong and (long other) {
        return new ULong(bits & other);
    }

    public ULong and (ULong other) {
        return and(other.bits);
    }

    public ULong or (long other) {
        return new ULong(bits | other);
    }

    public ULong or (ULong other) {
        return or(other.bits);
    }

    public ULong xor (long other) {
        return new ULong(bits ^ other);
    }

    public ULong xor (ULong other) {
        return xor(other.bits);
    }

    public ULong shiftLeft (int n) {
        return new ULong(SafeMath.shiftLeft(bits, n));
    }

    public ULong shiftRight (int n) {
        return new ULong(SafeMath.shiftRight(bits, n));
    }

    public boolean testBit (int n) {
        return n == 63 ? bits < 0 : ((bits >>> n) & 1) == 1;
    }

    public ULong setBit (int n, boolean value) {
        if (value) {
            return new ULong(bits | (1L << n));
        }

        return new ULong(bits & ~(1L << n));
    }

    public int leadingZeros () {
        return Long.numberOfLeadingZeros(bits);
    }

    public int trailingZeros () {
        return Long.numberOfTrailingZeros(bits);
    }

    // ARITHMETIC
    public ULong add (ULong other) {
        return new ULong(bits + other.bits);
    }

    public ULong subtr (ULong other) {
        return new ULong(bits - other.bits);
    }

    public ULong mul (ULong other) {
        return new ULong(bits * other.bits);
    }

    public ULong div (ULong other) {
        return new ULong(Long.divideUnsigned(bits, other.bits));
    }

    public ULong mod (ULong other) {
        return new ULong(Long.remainderUnsigned(bits, other.bits));
    }

    // COMPARE
    @Override
    public int compareTo (ULong o) {
        return Long.compareUnsigned(bits, o.bits);
    }

    public int compareTo (long o) {
        return o < 0 ? 1 : Long.compare(bits, o);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        ULong uLong = (ULong) o;
        return bits == uLong.bits;
    }

    @Override
    public int hashCode() {
        return Long.hashCode(bits);
    }

    // STRING
    public String toString (int radix) {
        return Long.toUnsignedString(bits, radix);
    }

    @Override
    public String toString() {
        return toString(10);
    }

    // NUMBER
    @Override
    public int intValue() {
        return (int) bits;
    }

    @Override
    public long longValue() {
        return bits;
    }

    @Override
    public float floatValue () {
        if (bits < 0) {
            return (float) (bits & 0x7fffffffffffffffL) + FLOAT_DELTA;
        }

        return (float) bits;
    }

    @Override
    public double doubleValue () {
        if (bits < 0) {
            return (double) (bits & 0x7fffffffffffffffL) + DOUBLE_DELTA;
        }

        return (double) bits;
    }

    // CLONE
    @Override
    public ULong clone () {
        return new ULong(bits);
    }

    // VALUE OF
    public static ULong valueOf (long value) {
        if (value < 0) {
            throw new NumberFormatException("Value passed is negative");
        }

        return new ULong(value);
    }

    public static ULong valueOf (String value, int radix) {
        return new ULong(Long.parseUnsignedLong(value, radix));
    }

    public static ULong valueOf (String value) {
        return valueOf(value, 10);
    }

    public static ULong ofBits (long bits) {
        return new ULong(bits);
    }
}

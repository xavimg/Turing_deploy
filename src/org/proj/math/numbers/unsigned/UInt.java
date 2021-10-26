package org.proj.math.numbers.unsigned;

import org.proj.math.SafeMath;

final public class UInt extends Number implements Cloneable, Comparable<UInt> {
    final public static UInt ZERO = valueOf(0);
    final public static UInt ONE = valueOf(1);
    final public static UInt TWO = valueOf(2);
    final public static UInt TEN = valueOf(10);

    final public static UInt MIN_VALUE = ZERO;
    final public static UInt MAX_VALUE = ofBits(-1);

    final private static double DOUBLE_DELTA = Math.scalb(1, 31);
    final private static float FLOAT_DELTA = Math.scalb(1, 31);

    final public int bits;

    private UInt(int bits) {
        this.bits = bits;
    }

    // BITWISE
    public UInt not () {
        return new UInt(~bits);
    }

    public UInt and (int other) {
        return new UInt(bits & other);
    }

    public UInt and (UInt other) {
        return and(other.bits);
    }

    public UInt or (int other) {
        return new UInt(bits | other);
    }

    public UInt or (UInt other) {
        return or(other.bits);
    }

    public UInt xor (int other) {
        return new UInt(bits ^ other);
    }

    public UInt xor (UInt other) {
        return xor(other.bits);
    }

    public UInt shiftLeft (int n) {
        return new UInt(SafeMath.shiftLeft(bits, n));
    }

    public UInt shiftRight (int n) {
        return new UInt(SafeMath.shiftRight(bits, n));
    }

    public boolean testBit (int n) {
        return n == 63 ? bits < 0 : ((bits >>> n) & 1) == 1;
    }

    public UInt setBit (int n, boolean value) {
        if (value) {
            return new UInt(bits | (1 << n));
        }

        return new UInt(bits & ~(1 << n));
    }

    public int leadingZeros () {
        return Long.numberOfLeadingZeros(bits);
    }

    public int trailingZeros () {
        return Long.numberOfTrailingZeros(bits);
    }

    // ARITHMETIC
    public UInt add (UInt other) {
        return new UInt(bits + other.bits);
    }

    public UInt subtr (UInt other) {
        return new UInt(bits - other.bits);
    }

    public int subtrs (UInt other) {
        return bits - other.bits;
    }

    public UInt mul (UInt other) {
        return new UInt(bits * other.bits);
    }

    public UInt div (UInt other) {
        return new UInt(Integer.divideUnsigned(bits, other.bits));
    }

    public UInt mod (UInt other) {
        return new UInt(Integer.remainderUnsigned(bits, other.bits));
    }

    // COMPARE
    @Override
    public int compareTo (UInt o) {
        return Integer.compareUnsigned(bits, o.bits);
    }

    public int compareTo (int o) {
        return o < 0 ? 1 : Integer.compare(bits, o);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        UInt uLong = (UInt) o;
        return bits == uLong.bits;
    }

    public boolean equals (UInt o) {
        if (this == o) return true;
        return bits == o.bits;
    }

    @Override
    public int hashCode() {
        return Integer.hashCode(bits);
    }

    // STRING
    public String toString (int radix) {
        return Integer.toUnsignedString(bits, radix);
    }

    @Override
    public String toString() {
        return toString(10);
    }

    // NUMBER
    @Override
    public int intValue() {
        return bits;
    }

    @Override
    public long longValue() {
        return bits;
    }

    @Override
    public float floatValue () {
        if (bits < 0) {
            return (float) (bits & 0x7fffffffL) + FLOAT_DELTA;
        }

        return (float) bits;
    }

    @Override
    public double doubleValue () {
        if (bits < 0) {
            return (double) (bits & 0x7fffffffL) + DOUBLE_DELTA;
        }

        return (double) bits;
    }

    // CLONE
    @Override
    public UInt clone () {
        return new UInt(bits);
    }

    // VALUE OF
    public static UInt valueOf (int value) {
        if (value < 0) {
            throw new NumberFormatException("Value passed is negative");
        }

        return new UInt(value);
    }

    public static UInt valueOf (String value, int radix) {
        return new UInt(Integer.parseUnsignedInt(value, radix));
    }

    public static UInt valueOf (String value) {
        return valueOf(value, 10);
    }

    public static UInt ofBits (int bits) {
        return new UInt(bits);
    }
}

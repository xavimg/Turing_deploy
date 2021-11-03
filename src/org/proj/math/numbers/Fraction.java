package org.proj.math.numbers;

public class Fraction extends Number {
    final private static double DOUBLE_DELTA = Math.scalb(1, 52);
    final public double num, denom;

    public Fraction (double num, double denom) {
        this.num = num;
        this.denom = denom;
    }

    public static Fraction valueOf (long value) {
        return new Fraction(value, 1);
    }

    public static Fraction valueOf (double value) {
        long alpha = (Double.doubleToLongBits(value) & 0xfffffffffffffL) | 0x10000000000000L;
        return new Fraction(alpha, Math.scalb(1, 52 - Math.getExponent(value)));
    }

    public Fraction add (Fraction other) {
        if (denom == other.denom) {
            return new Fraction(num + other.num, denom);
        }

        double denom = this.denom * other.denom;
        return new Fraction(this.num * other.denom + other.num * this.denom, denom);
    }

    public Fraction subtr (Fraction other) {
        if (denom == other.denom) {
            return new Fraction(num - other.num, denom);
        }

        double denom = this.denom * other.denom;
        return new Fraction(this.num * other.denom - other.num * this.denom, denom);
    }

    public Fraction mul (Fraction other) {
        return new Fraction(this.num * other.num, this.denom * other.denom);
    }

    public Fraction div (Fraction other) {
        return new Fraction(this.num * other.denom, this.denom * other.num);
    }

    // NUMBER
    @Override
    public int intValue() {
        return (int) doubleValue();
    }

    @Override
    public long longValue() {
        return (long) doubleValue();
    }

    @Override
    public float floatValue() {
        return (float) doubleValue();
    }

    @Override
    public double doubleValue() {
        return num / denom;
    }

    // OTHERS
    @Override
    public String toString() {
        return num + " / " + denom;
    }
}

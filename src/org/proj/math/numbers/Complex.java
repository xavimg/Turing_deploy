package org.proj.math.numbers;

public class Complex extends Number {
    final public static Complex ZERO = Complex.ofRe(0);
    final public static Complex ONE = Complex.ofRe(1);
    final public static Complex I = Complex.ofIm(1);

    final public double re, im;

    public Complex (double re, double im) {
        this.re = re;
        this.im = im;
    }

    public static Complex ofRe (double re) {
        return new Complex(re, 0);
    }

    public static Complex ofIm (double im) {
        return new Complex(0, im);
    }

    public static Complex ofPolar (double radius, double angle) {
        return new Complex(radius * Math.cos(angle), radius * Math.sin(angle));
    }

    // ARITHMETIC
    public Complex add (Complex other) {
        return new Complex(this.re + other.re, this.im + other.im);
    }

    public Complex add (double other) {
        return new Complex(this.re + other, this.im);
    }

    public Complex subtr (Complex other) {
        return new Complex(this.re - other.re, this.im - other.im);
    }

    public Complex subtr (double other) {
        return new Complex(this.re - other, this.im);
    }

    public Complex mul (Complex other) {
        return new Complex(this.re * other.re - this.im * other.im, this.re * other.im + this.im * other.re);
    }

    public Complex mul (double other) {
        return new Complex(this.re * other, this.im * other);
    }

    public Complex div (Complex other) {
        double alpha = other.re * other.re + other.im * other.im;
        return new Complex((this.re * other.re + this.im * other.im) / alpha, (this.re * other.im - this.im * other.re) / alpha);
    }

    public Complex div (double other) {
        return new Complex(this.re / other, this.im / other);
    }

    // METHODS
    public Complex negate () {
        return new Complex(-this.re, -this.im);
    }

    public double abs () {
        return Math.hypot(this.re, this.im);
    }

    public double angle () {
        return Math.atan2(this.im, this.re);
    }

    public Complex conj () {
        return new Complex(this.re, -this.im);
    }

    public Complex inverse () {
        double alpha = this.re * this.re + this.im * this.im;
        return new Complex(this.re / alpha, -this.im / alpha);
    }

    public Complex square () {
        return new Complex(this.re * this.re - this.im * this.im, 2 * this.re * this.im);
    }

    // NUMBER
    @Override
    public byte byteValue() {
        return (byte) this.re;
    }

    @Override
    public short shortValue() {
        return (short) this.re;
    }

    @Override
    public int intValue() {
        return (int) this.re;
    }

    @Override
    public long longValue() {
        return (long) this.re;
    }

    @Override
    public float floatValue() {
        return (float) this.re;
    }

    @Override
    public double doubleValue() {
        return this.re;
    }

    // OTHERS
    @Override
    public String toString() {
        if (this.im == 0) {
            return Double.toString(this.re);
        } else if (this.re == 0) {
            return this.im + "i";
        }

        return this.re + (this.im >= 0 ? " + " : " - ") + Math.abs(this.im);
    }
}

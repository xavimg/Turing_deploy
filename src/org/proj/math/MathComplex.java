package org.proj.math;

import org.proj.math.numbers.Complex;

public class MathComplex {
    final public static Complex SQRTI = new Complex(Math.PI / 4, Math.PI / 4);
    final private static double SQRT2_2 = Math.sqrt(2) / 2;

    public static Complex sqrt (double value) {
        return value >= 0 ? Complex.ofRe(Math.sqrt(value)) : Complex.ofIm(Math.sqrt(-value));
    }

    public static Complex sqrt (Complex value) {
        if (value.im == 0) {
            return sqrt(value.re);
        }

        double abs = value.abs();
        return new Complex(
                Math.sqrt((abs + value.re) / 2),
                Math.signum(value.im) * Math.sqrt((abs - value.re) / 2)
        );
    }

    public static Complex sqrti (double value) {
        double alpha = SQRT2_2 * Math.sqrt(Math.abs(value));
        return new Complex(alpha, -alpha);
    }

    public static Complex exp (Complex value) {
        return Complex.ofPolar(Math.exp(value.re), value.im);
    }

    public static Complex expi (double value) {
        return new Complex(Math.cos(value), Math.sin(value));
    }

    public static Complex log (double value) {
        return value >= 0 ? Complex.ofRe(Math.log(value)) : new Complex(Math.log(-value), Math.PI);
    }

    public static Complex log (Complex value) {
        return new Complex(Math.log(value.abs()), value.angle());
    }

    public static Complex logi (double value) {
        return value >= 0 ? new Complex(Math.log(value), MathUtils.HALF_PI) : new Complex(Math.log(-value), MathUtils.PI_3_2);
    }

    // TRIGONOMETRY
    public static Complex sin (Complex value) {
        return new Complex(Math.sin(value.re) * Math.cosh(value.im), Math.cos(value.re) * Math.sinh(value.im));
    }

    public static Complex sini (double other) {
        return Complex.ofIm(Math.sinh(other));
    }

    public static Complex cos (Complex value) {
        return new Complex(Math.cos(value.re) * Math.cosh(value.im), Math.sin(value.re) * Math.sinh(value.im));
    }

    public static Complex cosi (double value) {
        return Complex.ofRe(Math.cosh(value));
    }

    public static Complex tan (Complex other) {
        return sin(other).div(cos(other));
    }

    public static Complex tani (double other) {
        return Complex.ofIm(Math.tanh(other));
    }
}

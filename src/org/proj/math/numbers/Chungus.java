package org.proj.math.numbers;

import org.proj.math.numbers.unsigned.ULong;
import org.proj.utils.Compare;

import java.math.BigInteger;

public class Chungus extends Number implements Comparable<Chungus> {
    final public static Chungus ZERO = new Chungus(ULong.ZERO, ULong.ZERO);
    final public static Chungus ONE = new Chungus(ULong.ZERO, ULong.ONE);
    final public static Chungus TWO = new Chungus(ULong.ZERO, ULong.TWO);
    final public static Chungus TEN = new Chungus(ULong.ZERO, ULong.TEN);

    final private ULong left, right;

    public Chungus (ULong left, ULong right) {
        this.left = left;
        this.right = right;
    }

    // bITWISE
    public boolean sign () {
        return left.testBit(63);
    }

    public Chungus not () {
        return new Chungus(left.not(), right.not());
    }

    public Chungus and (Chungus other) {
        return new Chungus(left.and(other.left), right.and(other.right));
    }

    public Chungus or (Chungus other) {
        return new Chungus(left.or(other.left), right.or(other.right));
    }

    public Chungus xor (Chungus other) {
        return new Chungus(left.xor(other.left), right.xor(other.right));
    }

    public Chungus negate () {
        return not().add(ONE);
    }

    public Chungus abs () {
        return sign() ? negate() : this;
    }

    public Chungus shiftLeft (int n) {
        ULong left = this.left.shiftLeft(n).or(this.right.shiftRight(64 - n));
        ULong right = this.right.shiftLeft(n);

        return new Chungus(left, right);
    }

    public Chungus shiftRight (int n) {
        ULong left = this.left.shiftRight(n);
        ULong right = this.right.shiftRight(n).or(this.left.shiftLeft(64 - n));

        return new Chungus(left, right);
    }

    public boolean testBit (int n) {
        return n < 64 ? right.testBit(n) : left.testBit(n - 64);
    }

    public Chungus setBit (int n, boolean value) {
        Chungus mask = ONE.shiftLeft(n);
        return value ? or(mask) : and(mask.not());
    }

    public int leadingZeros () {
        int r = this.left.leadingZeros();
        return r < 64 ? r : 64 + this.right.leadingZeros();
    }

    public int leftMostBit () {
        return 128 - leadingZeros();
    }

    // ARITHMETIC
    public Chungus add (Chungus other) {
        ULong left = this.left.add(other.left);
        ULong[] right = safeAdd(this.right, other.right);

        return new Chungus(left.add(right[0]), right[1]);
    }

    public Chungus subtr (Chungus other) {
        return add(other.negate());
    }

    public Chungus mul (Chungus other) {
        Chungus right = safeMul(this.right, other.right);
        Chungus left = safeMul(this.right, other.left).add(safeMul(this.left, other.right)).shiftLeft(64);

        return right.add(left);
    }

    public Chungus div (Chungus other) {
        return divMod(other)[0];
    }

    public Chungus mod (Chungus other) {
        return divMod(other)[1];
    }

    public Chungus[] divMod (Chungus other) {
        if (other.equals(ZERO)) {
            throw new ArithmeticException("Division by zero");
        }

        boolean sign2 = other.sign();
        boolean sign = this.sign() ^ sign2;

        Chungus self = abs();
        other = other.abs();

        Chungus q = ZERO;
        Chungus r = ZERO;

        for (int i=this.leftMostBit();i>=0;i--) {
            r = r.shiftLeft(1).setBit(0, self.testBit(i));
            if (Compare.isGreaterOrEqual(r, other)) {
                r = r.subtr(other);
                q = q.setBit(i, true);
            }
        }

        return new Chungus[] { sign ? q.negate() : q, sign2 ? r.negate() : r };
    }

    // COMPARE
    @Override
    public int compareTo (Chungus o) {
        int comp = Long.compare(this.left.bits, o.left.bits);
        return comp == 0 ? this.right.compareTo(o.right) : comp;
    }

    // NUMBER
    @Override
    public int intValue() {
        return right.intValue();
    }

    @Override
    public long longValue() {
        return right.longValue();
    }

    public BigInteger bigIntegerValue () {
        return new BigInteger(toString());
    }

    @Override
    public float floatValue() {
        return right.floatValue() + Math.scalb((float) left.bits, 64);
    }

    @Override
    public double doubleValue() {
        return right.doubleValue() + Math.scalb((double) left.bits, 64);
    }

    // TO STRING
    @Override
    public String toString() {
        if (left.equals(ULong.ZERO)) {
            return right.toString();
        }

        boolean sign = this.sign();
        Chungus value = this.abs();
        StringBuilder builder = new StringBuilder();

        while (Compare.isGreater(value, ZERO)) {
            Chungus[] divMod = value.divMod(TEN);
            builder.append(divMod[1].intValue());
            value = divMod[0];
        }

        return (sign ? "-" : "") + builder.reverse().toString();
    }

    // VALUE OF
    public static Chungus valueOf (long value) {
        return new Chungus(value >= 0 ? ULong.ZERO : ULong.MAX_VALUE, ULong.ofBits(value));
    }

    public static Chungus valueOf (ULong value) {
        return new Chungus(ULong.ZERO, value);
    }

    public static Chungus valueOf (ULong... array) {
        int delta = array.length - 2;
        return new Chungus(array[delta], array[delta+1]);
    }

    // SAFE ARITHMETIC
    private static ULong[] safeAdd (ULong alpha, ULong beta) {
        ULong r = alpha.and(0x7fffffffffffffffL).add(beta.and(0x7fffffffffffffffL));
        ULong l = alpha.shiftRight(63).add(beta.shiftRight(63));

        ULong sum = r.shiftRight(63).add(l);
        return new ULong[] { sum.shiftRight(1), r.setBit(63, sum.testBit(0)) };
    }

    private static Chungus safeMul (ULong alpha, ULong beta) {
        ULong a = alpha.and(0xffffffffL);
        ULong b = alpha.shiftRight(32);
        ULong x = beta.and(0xffffffffL);
        ULong y = beta.shiftRight(32);

        Chungus q = valueOf(a.mul(x));
        Chungus w = valueOf(safeAdd(a.mul(y), b.mul(x))).shiftLeft(32);
        Chungus e = valueOf(b.mul(y)).shiftLeft(64);

        return q.add(w).add(e);
    }
}

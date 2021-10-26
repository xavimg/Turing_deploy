package org.proj.math.numbers;

import org.proj.math.numbers.unsigned.ULong;

public class SafeSum<T extends Number> {
    final boolean overflow;
    final T result;

    private SafeSum (boolean overflow, T result) {
        this.overflow = overflow;
        this.result = result;
    }

    public static SafeSum<Integer> ofInt (int x, int y) {
        int r = x + y;
        return new SafeSum<>(((x ^ r) & (y ^ r)) < 0, r);
    }

    public static SafeSum<Long> ofLong (long x, long y) {
        long r = x + y;
        return new SafeSum<>(((x ^ r) & (y ^ r)) < 0, r);
    }

    public static SafeSum<ULong> ofULong (ULong x, ULong y) {
        ULong r = x.add(y);
        return new SafeSum<>(x.xor(r).add(y.xor(r)).testBit(63), r);
    }

    public static SafeSum<Chungus> ofChungus (Chungus x, Chungus y) {
        Chungus r = x.add(y);
        return new SafeSum<>(x.xor(r).add(y.xor(r)).compareTo(Chungus.ZERO) < 0, r);
    }
}

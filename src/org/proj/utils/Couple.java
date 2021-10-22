package org.proj.utils;

import java.util.function.Function;

public class Couple<A,B> {
    final public A first;
    final public B last;

    public Couple (A first, B last) {
        this.first = first;
        this.last = last;
    }

    public <X,Y> Couple<X,Y> map (Function<A, X> alpha, Function<B, Y> beta) {
        return new Couple<>(alpha.apply(first), beta.apply(last));
    }

    @Override
    public String toString() {
        return "Couple{" +
                "first=" + first +
                ", last=" + last +
                '}';
    }
}

package org.proj.utils;

public class Couple<A,B> {
    final public A first;
    final public B last;

    public Couple (A first, B last) {
        this.first = first;
        this.last = last;
    }

    @Override
    public String toString() {
        return "Couple{" +
                "first=" + first +
                ", last=" + last +
                '}';
    }
}

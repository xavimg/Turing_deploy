package org.proj.game.factory;

import org.proj.game.BlackHole;

import java.util.Random;
import java.util.function.Supplier;

public class BlackHoleFactory implements Supplier<BlackHole> {
    final public Random random;

    public BlackHoleFactory (Random random) {
        this.random = random;
    }

    public BlackHoleFactory () {
        this(new Random());
    }

    @Override
    public BlackHole get () {
        return null;
    }
}

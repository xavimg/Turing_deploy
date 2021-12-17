package org.proj.game.body.factory;

import org.proj.game.PlanetarySystem;
import org.proj.game.body.SpaceBody;
import org.proj.math.RandUtils;
import org.proj.utils.Range;

import java.util.Random;
import java.util.function.Supplier;

public class PlanetarySystemFactory implements Supplier<PlanetarySystem> {
    final public Random random;
    final public SpaceBodyFactory spaceBody;

    public PlanetarySystemFactory (Random random, SpaceBodyFactory spaceBody) {
        this.random = random;
        this.spaceBody = spaceBody;
    }

    public PlanetarySystemFactory (Random random) {
        this(random, new SpaceBodyFactory(random));
    }

    public PlanetarySystemFactory () {
        this(new Random());
    }

    private int getPlanetCount () {
        double n = RandUtils.nextGaussian(random, 3.44274809160305, 0.974172620904933);
        return Math.max(1, (int) Math.round(n));
    }

    @Override
    public PlanetarySystem get () {
        int n = getPlanetCount();

        Range.ofInt(0, n, true).forEach(i -> {
            SpaceBody body = this.spaceBody.get();
        });

        return null;
    }
}

package org.proj.game.body.factory;

import org.proj.game.body.SpaceBody;

import java.util.Random;
import java.util.function.Supplier;

public class SpaceBodyFactory implements Supplier<SpaceBody> {
    final public Random random;
    final public PlanetFactory planet;
    final public SunFactory sun;

    public SpaceBodyFactory (Random random, PlanetFactory planet, SunFactory sun) {
        this.random = random;
        this.planet = planet;
        this.sun = sun;
    }

    public SpaceBodyFactory (Random random) {
        this.random = random;
        this.planet = new PlanetFactory(random);
        this.sun = new SunFactory(random);
    }

    @Override
    public SpaceBody get () {
        boolean isPlanet = random.nextDouble() <= 0.774914089347079;
        return isPlanet ? planet.get() : sun.get();
    }
}

package org.proj.game.factory;

import org.proj.game.Planet;
import org.proj.game.PlanetarySystem;
import org.proj.game.Sun;
import org.proj.math.Range;

import java.util.Random;
import java.util.Vector;
import java.util.function.Supplier;

public class PlanetarySystemFactory implements Supplier<PlanetarySystem> {
    final public Random random;
    final public PlanetFactory planet;
    final public SunFactory sun;

    public PlanetarySystemFactory(Random random, PlanetFactory planet, SunFactory sun) {
        this.random = random;
        this.planet = planet;
        this.sun = sun;
    }

    public PlanetarySystemFactory (Random random) {
        this.random = random;
        this.planet = new PlanetFactory(random);
        this.sun = new SunFactory(random);
    }

    public PlanetarySystemFactory () {
        this(new Random());
    }

    private int getPlanetCount () {
        double n = random.nextGaussian(3.44274809160305, 0.974172620904933);
        return Math.max(1, (int) Math.round(n));
    }

    @Override
    public PlanetarySystem get () {
        Sun sun = this.sun.get();
        int n = getPlanetCount();

        Range.ofInt(0, n, true).forEach(i -> {
            Planet planet = this.planet.get();
            //Vector position = random.nextDouble(0, 1);
        });

        return null;
    }
}

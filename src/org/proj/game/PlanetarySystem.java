package org.proj.game;

import org.proj.math.Range;
import org.proj.math.vector.LazyVector;
import org.proj.math.vector.Vector;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.utils.Couple;

import java.util.Arrays;
import java.util.concurrent.atomic.AtomicReference;

public class PlanetarySystem {
    final private Sun central;
    final private Planet[] planets;

    public PlanetarySystem (Sun central, Planet... planets) {
        this.central = central;
        this.planets = planets;
    }

    public void step (double dt) {
        Vector[] delta = new Vector[planets.length];

        Range.ofInt(0, planets.length, true).forEach(i -> {
            Planet planet = planets[i];
            delta[i] = central.metric.getAcceleration(planet).mul(dt);

            // TODO INTERPLANETARY GRAVITY
        });

        Range.ofInt(0, planets.length, true).forEach(i -> {
            Planet planet = planets[i];
            planet.addVelocity(delta[i]);
            planet.update(dt);
        });
    }
}

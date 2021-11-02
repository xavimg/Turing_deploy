package org.proj.game;

import org.proj.math.Range;
import org.proj.math.vector.Vector;

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
            Vector pos = central.metric.getCoordinateSystem().fromCartesianPosition(planet.getPosition());
            Vector vel = central.metric.getCoordinateSystem().fromCartesianVelocity(planet.getPosition(), planet.getVelocity());

            Vector deltaVel = central.metric.getAcceleration(planet.delta(pos, vel)).mul(dt);
            delta[i] = central.metric.getCoordinateSystem().toCartesianVelocity(pos, deltaVel);
        });

        Range.ofInt(0, planets.length, true).forEach(i -> {
            Planet planet = planets[i];
            planet.addVelocity(delta[i]);
            planet.update(dt);
        });
    }
}

package org.proj.game;

import org.proj.math.Range;
import org.proj.math.vector.LazyVector;
import org.proj.math.vector.Vector;
import org.proj.math.vector.special.ZeroVector;

import java.util.concurrent.atomic.AtomicReference;

public class PlanetarySystem {
    final private SpaceBody[] bodies;

    public PlanetarySystem(SpaceBody... bodies) {
        this.bodies = bodies;
    }

    public void step (double dt) {
        Vector[] delta = new Vector[bodies.length];

        Range.ofInt(0, bodies.length, true).forEach(i -> {
            SpaceBody body = bodies[i];
            AtomicReference<Vector> acc = new AtomicReference<>(new ZeroVector(2));

            Range.ofInt(0, bodies.length, true).filter(j -> j != i).forEach(j -> {
                SpaceBody central = bodies[j];
                Vector pos = body.getPosition().subtr(central.getPosition());
                Vector vel = body.getVelocity().subtr(central.getVelocity());
                acc.updateAndGet(x -> x.add(central.metric.getAcceleration(body.delta(pos, vel))));
            });

            delta[i] = acc.get().mul(dt);
        });

        Range.ofInt(0, bodies.length, true).forEach(i -> {
            SpaceBody body = bodies[i];
            body.addVelocity(delta[i]);
            body.update(dt);
        });
    }
}

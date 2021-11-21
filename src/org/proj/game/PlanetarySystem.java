package org.proj.game;

import org.proj.math.vector.Vec2;
import org.proj.utils.Range;

import java.util.concurrent.atomic.AtomicReference;

public class PlanetarySystem {
    final private SpaceBody[] bodies;

    public PlanetarySystem(SpaceBody... bodies) {
        this.bodies = bodies;
    }

    public void step (double dt) {
        Vec2[] delta = new Vec2[bodies.length];

        Range.ofInt(0, bodies.length, true).forEach(i -> {
            SpaceBody body = bodies[i];
            AtomicReference<Vec2> acc = new AtomicReference<>(Vec2.ZER0);

            Range.ofInt(0, bodies.length, true).filter(j -> j != i).forEach(j -> {
                SpaceBody central = bodies[j];
                Vec2 pos = body.getPosition().subtr(central.getPosition());
                Vec2 vel = body.getVelocity().subtr(central.getVelocity());
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

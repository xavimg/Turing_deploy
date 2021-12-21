package org.proj.game;

import org.proj.game.body.Planet;
import org.proj.game.body.SpaceBody;
import org.proj.game.body.Sun;
import org.proj.game.resource.PTElement;
import org.proj.math.vector.Vec2;
import org.proj.utils.Range;
import org.rol.ReadOnlyList;

import java.util.concurrent.atomic.AtomicReference;

public class PlanetarySystem extends ReadOnlyList<SpaceBody> {
    final private Sun[] suns;
    final private Planet[] planets;

    public PlanetarySystem (Sun[] suns, Planet... planets) {
        this.suns = suns;
        this.planets = planets;
    }

    public PlanetarySystem (SpaceBody... bodies) {
        this.suns = Range.ofArray(bodies, true)
                .filter(x -> x instanceof Sun)
                .map(x -> (Sun) x)
                .toArray(Sun[]::new);

        this.planets = Range.ofArray(bodies, true)
                .filter(x -> x instanceof Planet)
                .map(x -> (Planet) x)
                .toArray(Planet[]::new);
    }

    public void step (double dt) {
        Vec2[] delta = new Vec2[size()];

        Range.ofInt(0, size(), true).forEach(i -> {
            SpaceBody body = get(i);
            AtomicReference<Vec2> acc = new AtomicReference<>(Vec2.ZER0);

            Range.ofInt(0, size(), true).filter(j -> j != i).forEach(j -> {
                SpaceBody central = get(j);
                Vec2 pos = body.getPosition().subtr(central.getPosition());
                Vec2 vel = body.getVelocity().subtr(central.getVelocity());
                acc.updateAndGet(x -> x.add(central.metric.getAcceleration(body.delta(pos, vel))));
            });

            delta[i] = acc.get().mul(dt);
        });

        Range.ofInt(0, size(), true).forEach(i -> {
            SpaceBody body = get(i);
            body.addVelocity(delta[i]);
            body.update(dt);
        });
    }

    public Sun[] getSuns () {
        return suns.clone();
    }

    public Planet[] getPlanets () {
        return planets.clone();
    }

    @Override
    public SpaceBody get (int i) {
        return i < suns.length ? suns[i] : planets[i - suns.length];
    }

    @Override
    public int size() {
        return suns.length + planets.length;
    }
}

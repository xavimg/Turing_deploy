package org.proj.game.body.factory;

import org.proj.game.body.Planet;
import org.proj.math.MathUtils;
import org.proj.math.RandUtils;
import org.proj.physics.metric.Schwarzschild;

import java.awt.*;
import java.util.Random;
import java.util.function.Supplier;

public class PlanetFactory implements Supplier<Planet> {
    final public static double MIN_MASS = 1.01e-9;
    final public static double MIN_RADIUS = 2.13e-4;

    final public static double MAX_MASS = 0.01146;
    final public static double MAX_RADIUS = 2.289;

    final public Random random;
    final public TextureFactory texture;

    public PlanetFactory (Random random, TextureFactory texture) {
        this.random = random;
        this.texture = texture;
    }

    public PlanetFactory (Random random) {
        this.random = random;
        this.texture = new TextureFactory(random);
    }

    public PlanetFactory() {
        this(new Random());
    }

    private double getRestMass (double min, double max) {
        double jupiter = RandUtils.nextGaussian(random, 1.81770640331076d, 4.01265200872978d);
        return MathUtils.clamp(jupiter * 9.55e-4, min, max);
    }

    private double getRadius (double min, double max) {
        double jupiter = RandUtils.nextGaussian(random, 205.53565090755600d, 181.28364133659100d);
        return MathUtils.clamp(jupiter * 0.2385, min, max);
    }

    @Override
    public Planet get () {
        double mass = getRestMass(MIN_MASS, MAX_MASS);
        double radius = getRadius(Schwarzschild.radius(mass) + MIN_RADIUS, MAX_RADIUS);

        Color color = new Color(random.nextInt()); // TODO TEMPORAL
        Image texture = this.texture.get(); // TODO TEMPORAL

        return new Planet(mass, radius, null, null, color, texture);
    }
}
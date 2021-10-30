package org.proj.game.factory;

import org.proj.game.Sun;
import org.proj.math.MathUtils;
import org.proj.physics.Constants;

import java.util.Random;
import java.util.function.Supplier;

public class SunFactory implements Supplier<Sun> {
    final public TextureFactory texture;
    final public Random random;

    public SunFactory (TextureFactory texture, Random random) {
        this.texture = texture;
        this.random = random;
    }

    public SunFactory (Random random) {
        this.random = random;
        this.texture = new TextureFactory(random);
    }

    public SunFactory () {
        this(new Random());
    }

    @Override
    public Sun get () {
        double temperature = random.nextGaussian(2919.21117764983, 1073.60724180104);
        double radius = 0.725841 * Math.exp(0.000073683 * temperature) * 2.32061;

        double lambda = Constants.C / radius;
        double angularVelocity = random.nextGaussian(lambda / 2, lambda / 5);

        return new Sun(temperature, radius, MathUtils.clamp(angularVelocity, 0, lambda));
    }
}

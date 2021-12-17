package org.proj.game.body.factory;

import org.proj.utils.Range;
import org.proj.utils.PerlinNoise;

import java.awt.*;
import java.awt.image.BufferedImage;
import java.util.Random;
import java.util.function.Supplier;

public class TextureFactory implements Supplier<Image> {
    final public Random random;
    final public PerlinNoise noise;

    public TextureFactory (Random random, PerlinNoise noise) {
        this.random = random;
        this.noise = noise;
    }

    public TextureFactory(Random random) {
        this.random = random;
        this.noise = new PerlinNoise(random);
    }

    public TextureFactory () {
        this(new Random());
    }

    @Override
    public Image get () {
        double x = random.nextDouble();
        double y = random.nextDouble();

        BufferedImage image = new BufferedImage(128, 128, BufferedImage.TYPE_INT_RGB);
        Range.ofInt(0, 128, true).forEach(i -> {
            double k = 0.05 * (x + i);
            Range.ofInt(0, 128, true).forEach(j -> {
                double value = noise.noise2(k, 0.05 * (y + j)) + 1;
                int rgb = (int) Math.round(127.5d * value) & 0xFF;

                image.setRGB(i, j, rgb | (rgb << 8) | (rgb << 16));
            });
        });

        return image;
    }
}

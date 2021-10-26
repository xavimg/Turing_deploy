package org.proj.game.factory;

import java.awt.*;
import java.util.Random;
import java.util.function.Supplier;

public class TextureFactory implements Supplier<Image> {
    final public Random random;

    public TextureFactory (Random random) {
        this.random = random;
    }

    @Override
    public Image get () {
        return null;
    }
}

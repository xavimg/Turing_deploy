import org.junit.Test;
import org.proj.utils.Range;
import org.proj.utils.PerlinNoise;

import java.awt.*;
import java.awt.image.BufferedImage;

public class NoiseTest {
    @Test
    public void testRandom() {
        PerlinNoise alpha = new PerlinNoise(1234);
        BufferedImage image = new BufferedImage(500, 500, BufferedImage.TYPE_INT_RGB);

        Range.ofInt(0, 500, true).forEach(i -> Range.ofInt(0, 500, true).forEach(j -> {
            double w = alpha.noise2(i * 0.05d, j * 0.05d);
            int rgb = (int) Math.round(255 * (w + 1) / 2);

            image.setRGB(i, j, new Color(rgb, rgb, rgb).getRGB());
        }));

        System.out.println();
    }
}

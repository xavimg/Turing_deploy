import org.junit.Test;
import org.proj.game.Planet;
import org.proj.game.Sun;
import org.proj.math.MathUtils;
import org.proj.math.matrix.Matrix;
import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.metric.Schwarzschild;
import org.proj.physics.metric.cartesian.SchwarzschildCartesian;
import org.proj.utils.ThreadUtils;

import javax.swing.*;
import java.awt.*;

public class TransformTest {
    public static void main (String... args) {
        SchwarzschildCartesian metric = new SchwarzschildCartesian(1d);
        Planet earth = new Planet(3e-6, 0.021251398d, 7.292115e-5, Vector.of(496.6d, 0), Vector.of(0, 9.93e-5), null, null);
        Planet newton = new Planet(3e-6, 0.021251398d, 7.292115e-5, Vector.of(496.6d, 0), Vector.of(0, 9.93e-5), null, null);

        JFrame window = new PaintedWindow("Schwarzschild test") {
            @Override
            public void paint (Graphics g) {
                int midX = getWidth() / 2;
                int midY = getHeight() / 2;

                double weight = 496.6d / getWidth();
                Vector pos1 = newton.getPosition().mul(weight);
                Vector pos2 = earth.getPosition().mul(weight);

                g.setColor(Color.BLACK);
                g.fillOval(midX - 50, midY - 50, 100, 100);

                g.setColor(new Color(255, 0, 0, 128));
                g.fillOval((int) Math.round(pos1.get(0) + midX - 26), (int) Math.round(pos1.get(1) + midY - 26), 52, 52);

                g.setColor(new Color(0, 255, 0, 128));
                g.fillOval((int) Math.round(pos2.get(0) + midX - 25), (int) Math.round(pos2.get(1) + midY - 25), 50, 50);
            }
        };

        Thread update = ThreadUtils.interval(10, () -> {
            double sec = (10 * 1e-3) * 60 * 60 * 24 * 7 * 4; // Every second = 1 month

            Vector acc = metric.getAcceleration(earth);
            earth.addVelocity(acc.mul(sec));
            earth.update(sec);

            double r2 = newton.getPosition().length2();
            double newtonAcc = Constants.G * metric.mass / r2;
            Vector newtonDir = newton.getPosition().mul(-1).div(Math.sqrt(r2));

            newton.addVelocity(newtonDir.mul(newtonAcc).mul(sec));
            newton.update(sec);

            window.repaint();
        });

        window.setSize(900, 1500);
        window.setVisible(true);
        window.createBufferStrategy(2);
        update.start();
    }
}

import org.junit.Test;
import org.proj.game.Planet;
import org.proj.game.PlanetarySystem;
import org.proj.game.Sun;
import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.physics.coordinate.Polar;
import org.proj.utils.ThreadUtils;

import javax.swing.*;
import java.awt.*;

public class SystemTest {
    public static void main (String... args) {
        // 9.93e-5
        // 1.17e-4

        Sun sun = new Sun(1d, 2.321d, 2.904e-6, 0);
        Planet earth = new Planet(3e-6, 0.021251398d, 7.292115e-5, Vector.of(496.6d, 0), Vector.of(0, 9.93e-5), null, null);
        Planet venus = new Planet(2.448e-6, 0.0202, 2.99246e-7, Vector.of(362.8d, 0), Vector.of(0, 1.17e-4), null, null);

        Planet newtonEarth = new Planet(3e-6, 0.021251398d, 7.292115e-5, Vector.of(496.6d, 0), Vector.of(0, 9.93e-5), null, null);
        PlanetarySystem system = new PlanetarySystem(sun, venus, earth);

        JFrame window = new PaintedWindow("Schwarzschild test") {
            @Override
            public void paint(Graphics g) {
                int midX = getWidth() / 2;
                int midY = getHeight() / 2;

                double weight = 496.6d / getWidth();
                Vector pos1 = venus.getPosition().mul(weight);
                Vector pos2 = earth.getPosition().mul(weight);

                Vector posn2 = newtonEarth.getPosition().mul(weight);

                g.setColor(Color.BLACK);
                g.fillOval(midX - 50, midY - 50, 100, 100);

                g.setColor(new Color(128, 200, 0, 128));
                g.fillOval((int) Math.round(pos1.get(0) + midX - 25), (int) Math.round(pos1.get(1) + midY - 25), 50, 50);

                g.setColor(new Color(0, 0, 255, 128));
                g.fillOval((int) Math.round(pos2.get(0) + midX - 25), (int) Math.round(pos2.get(1) + midY - 25), 50, 50);

                // NEWTONIAN
                g.setColor(new Color(128, 128, 255, 128));
                g.fillOval((int) Math.round(posn2.get(0) + midX - 25), (int) Math.round(posn2.get(1) + midY - 25), 50, 50);
            }
        };

        Thread update = ThreadUtils.interval(10, () -> {
            double sec = (10 * 1e-3) * 60 * 60 * 24 * 30; // Every second = 1 month
            system.step(sec);

            // NEWTON BASE EXAMPLE
            double r2 = newtonEarth.getPosition().length2();
            double newtonAcc = Constants.G * sun.restMass() / r2;
            Vector newtonDir = newtonEarth.getPosition().mul(-1).div(Math.sqrt(r2));

            newtonEarth.addVelocity(newtonDir.mul(newtonAcc).mul(sec));
            newtonEarth.update(sec);

            window.repaint();
        });

        window.setSize(900, 900);
        window.setVisible(true);
        window.createBufferStrategy(2);
        update.start();
    }
}

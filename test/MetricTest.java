import org.proj.game.Planet;
import org.proj.math.vector.Vector;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.physics.metric.Schwarzschild;
import org.proj.utils.ThreadUtils;

import javax.swing.*;
import java.awt.*;

public class MetricTest {
    public static void main (String... args) {
        double mass = 1d;
        Schwarzschild metric = new Schwarzschild(mass);

        Planet cartesian = new Planet(3e-6, 0.021251398d, 7.292115e-5, Vector.of(496.6d, 0), Vector.of(0, 9.93e-5), null, null);
        Planet polar = new Planet(3e-6, 0.021251398d, 7.292115e-5, Vector.of(496.6d, 0), CoordinateSystem.POLAR.fromCartesianVelocity(Vector.of(496.6d, 0), Vector.of(0, 9.93e-5)), null, null);

        JFrame window = new PaintedWindow("Schwarzschild test") {
            @Override
            public void paint(Graphics g) {
                int midX = getWidth() / 2;
                int midY = getHeight() / 2;

                double weight = 496.6d / getWidth();
                Vector pos1 = cartesian.getPosition().mul(weight);
                Vector pos2 = CoordinateSystem.POLAR.toCartesianPosition(polar.getPosition()).mul(weight);

                g.setColor(Color.BLACK);
                g.fillOval(midX - 50, midY - 50, 100, 100);

                // CARTESIAN
                g.setColor(new Color(0, 0, 255, 128));
                g.fillOval((int) Math.round(pos1.get(0) + midX - 25), (int) Math.round(pos1.get(1) + midY - 25), 50, 50);

                // POLAR
                g.setColor(new Color(128, 128, 255, 128));
                g.fillOval((int) Math.round(pos2.get(0) + midX - 25), (int) Math.round(pos2.get(1) + midY - 25), 50, 50);
            }
        };

        Thread update = ThreadUtils.interval(1, () -> {
            double sec = (1 * 1e-3) * 60 * 60 * 24 * 30;
            polar.addVelocity(metric.getAcceleration(polar).mul(sec));
            polar.update(sec);

            Vector pos = CoordinateSystem.POLAR.fromCartesianPosition(cartesian.getPosition());
            Vector vel = CoordinateSystem.POLAR.fromCartesianVelocity(cartesian.getPosition(), cartesian.getVelocity());
            Vector acc = metric.getAcceleration(cartesian.delta(pos, vel));

            System.out.println(acc);
            System.out.println(metric.getAcceleration(polar));
            System.out.println();

            cartesian.addVelocity(CoordinateSystem.POLAR.toCartesianVelocity(pos, acc.mul(sec)));
            cartesian.update(sec);

            window.repaint();
        });

        window.setSize(900, 1500);
        window.setVisible(true);
        window.createBufferStrategy(2);
        update.start();
    }
}

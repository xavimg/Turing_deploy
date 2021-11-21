import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.physics.metric.Schwarzschild;
import org.proj.utils.ThreadUtils;

import javax.swing.*;
import java.awt.*;

public class SchwrzTest {
    public static void main (String... args) {
        double mass = 1;
        double r = 496.6d;
        double velocity = 9.93e-5;

        double radius = 2.321d;
        double av = 1d; // Half C
        //double av = 2.904e-6;

        Schwarzschild schwarzschild = new Schwarzschild(mass);
        Kerr kerr = new Kerr(mass, radius, av);

        Matter.Defined sEarth = new Matter.Defined(3e-6, 0.021251398d, 7.292115e-5, Vector.of(r, 0), CoordinateSystem.POLAR.fromCartesianVelocity(Vector.of(r, 0), Vector.of(0, velocity)));
        Matter.Defined kEarth = new Matter.Defined(3e-6, 0.021251398d, 7.292115e-5, sEarth.getPosition(), sEarth.getVelocity());
        Matter.Defined newton = new Matter.Defined(3e-6, 0.021251398d, 7.292115e-5, Vector.of(r, 0), Vector.of(0, velocity));

        JFrame window = new PaintedWindow("Schwarzschild test") {
            @Override
            public void paint (Graphics g) {
                int midX = getWidth() / 2;
                int midY = getHeight() / 2;

                double weight = r / getWidth();
                Vector pos1 = newton.getPosition().mul(weight);
                Vector pos2 = schwarzschild.getCoordinateSystem().toCartesianPosition(sEarth.getPosition()).mul(weight);
                Vector pos3 = kerr.toCartesianPosition(kEarth.getPosition()).mul(weight);

                g.setColor(Color.BLACK);
                g.fillOval(midX - 50, midY - 50, 100, 100);

                g.setColor(new Color(255, 0, 0, 128));
                g.fillOval((int) Math.round(pos1.get(0) + midX - 25), (int) Math.round(pos1.get(1) + midY - 25), 50, 50);

                g.setColor(new Color(255, 255, 0, 128));
                g.fillOval((int) Math.round(pos2.get(0) + midX - 25), (int) Math.round(pos2.get(1) + midY - 25), 50, 50);

                g.setColor(new Color(0, 255, 0, 128));
                g.fillOval((int) Math.round(pos3.get(1) + midX - 25), (int) Math.round(pos3.get(0) + midY - 25), 50, 50);
            }
        };

        Thread update = ThreadUtils.interval(10, () -> {
            double sec = (10 * 1e-3) * 60 * 60 * 24 * 7 * 4; // Every second = 1 month

            Vector acc = schwarzschild.getAcceleration(sEarth);
            sEarth.addVelocity(acc.mul(sec));
            sEarth.update(sec);

            Vector acc2 = kerr.getAcceleration(kEarth);
            kEarth.addVelocity(acc2.mul(sec));
            kEarth.update(sec);

            double r2 = newton.getPosition().length2();
            double newtonAcc = Constants.G * mass / r2;
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

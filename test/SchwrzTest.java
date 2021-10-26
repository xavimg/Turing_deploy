import org.proj.math.MathUtils;
import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.physics.metric.Schwarzschild;
import org.proj.utils.ThreadUtils;

import javax.swing.*;
import java.awt.*;
import java.math.BigDecimal;
import java.math.MathContext;

public class SchwrzTest {
    public static void main (String... args) {
        BigDecimal mass = BigDecimal.ONE;
        BigDecimal r = BigDecimal.valueOf(496.6d);
        BigDecimal velocity = BigDecimal.valueOf(9.93e-5);

        Schwarzschild metric = new Schwarzschild(mass);
        Matter.Defined earth = new Matter.Defined(BigDecimal.valueOf(3e-6), BigDecimal.valueOf(0.021251398d), BigDecimal.valueOf(7.292115e-5), Vector.of(r, BigDecimal.ZERO), CoordinateSystem.POLAR.fromCartesianVelocity(Vector.of(r, BigDecimal.ZERO), Vector.of(BigDecimal.ZERO, velocity)));
        Matter.Defined newton = new Matter.Defined(BigDecimal.valueOf(3e-6), BigDecimal.valueOf(0.021251398d), BigDecimal.valueOf(7.292115e-5), Vector.of(r, BigDecimal.ZERO), Vector.of(BigDecimal.ZERO, velocity));

        JFrame window = new PaintedWindow("Schwarzschild test") {
            @Override
            public void paint (Graphics g) {
                g.clearRect(0, 0, Integer.MAX_VALUE, Integer.MAX_VALUE);

                int midX = getWidth() / 2;
                int midY = getHeight() / 2;

                BigDecimal weight = r.multiply(MathUtils.TWO).divide(BigDecimal.valueOf(getWidth()), MathContext.DECIMAL128);
                Vector pos1 = metric.getCoordinateSystem().toCartesianPosition(earth.getPosition()).mul(weight);
                Vector pos2 = newton.getPosition().mul(weight);

                g.setColor(Color.BLACK);
                g.fillOval(midX - 50, midY - 50, 100, 100);

                g.setColor(new Color(255, 0, 0, 128));
                g.fillOval(pos2.get(0).intValue() + midX - 25, pos2.get(1).intValue() + midY - 25, 50, 50);

                g.setColor(new Color(0, 255, 0, 128));
                g.fillOval(pos1.get(0).intValue() + midX - 25, pos1.get(1).intValue() + midY - 25, 50, 50);
                //System.out.println();
            }
        };

        window.setSize(900, 1500);
        window.setVisible(true);

        Thread paint = ThreadUtils.interval(33, (t) -> {
            window.repaint();
        });

        paint.start();

        long start = System.nanoTime();
        while (true) {
            long end = System.nanoTime();
            double delta = end - start;

            BigDecimal sec = BigDecimal.valueOf((delta * 1e-9) * 60 * 60 * 24 * 7); // Every second = a week

            Vector acc = metric.getAcceleration(earth);
            earth.addVelocity(acc.mul(sec));
            earth.update(sec);

            BigDecimal r2 = newton.getPosition().length2();
            BigDecimal newtonAcc = Constants.G.multiply(mass).divide(r2, MathContext.DECIMAL128);
            Vector newtonDir = newton.getPosition().mul(BigDecimal.ONE.negate()).div(r2.sqrt(MathContext.DECIMAL128));

            newton.addVelocity(newtonDir.mul(newtonAcc).mul(sec));
            newton.update(sec);
            start = end;
        }
    }
}

import org.proj.game.Planet;
import org.proj.math.vector.Vec2;
import org.proj.physics.Constants;
import org.proj.physics.metric.SchwarzschildCartesian;
import org.proj.utils.ThreadUtils;

import javax.swing.*;
import java.awt.*;

public class TransformTest {
    public static void main (String... args) {
        SchwarzschildCartesian metric = new SchwarzschildCartesian(1d);
        Planet mercury = new Planet(1.675e-55, 0.00814, new Vec2(117.7d, 0), new Vec2(0, 1.58e-4), null, null);
        Planet newton = new Planet(1.675e-55, 0.00814, new Vec2(117.7d, 0), new Vec2(0, 1.58e-4), null, null);

        JFrame window = new PaintedWindow("Schwarzschild test") {
            @Override
            public void paint (Graphics g) {
                int midX = getWidth() / 2;
                int midY = getHeight() / 2;

                double weight = 2000d / getWidth();
                Vec2 pos1 = newton.getPosition().mul(weight);
                Vec2 pos2 = mercury.getPosition().mul(weight);

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

            Vec2 acc = metric.getAcceleration(mercury);
            mercury.addVelocity(acc.mul(sec));
            mercury.update(sec);

            double r2 = newton.getPosition().length2();
            double newtonAcc = Constants.G * metric.mass / r2;
            Vec2 newtonDir = newton.getPosition().mul(-1).div(Math.sqrt(r2));

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

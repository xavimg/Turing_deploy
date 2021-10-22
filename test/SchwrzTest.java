import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.physics.metric.Schwarzschild;

import java.math.BigDecimal;

public class SchwrzTest {
    public static void main (String... args) {
        double mass = 1d;
        double r = 496.6d;
        double velocity = 9.93e-5;

        Schwarzschild metric = new Schwarzschild(mass);

        Matter.Defined earth = new Matter.Defined(3e-6, 0.021251398d, 7.292115e-5, Vector.of(r, 0), CoordinateSystem.POLAR.fromCartesianVelocity(Vector.of(r, 0), Vector.of(0, velocity)));
        Matter.Defined newton = new Matter.Defined(3e-6, 0.021251398d, 7.292115e-5, Vector.of(r, 0), Vector.of(0, velocity));

        Vector start = earth.getPosition();
        System.out.println(start);
        System.out.println();

        for (int i=0;i<87600;i++) { // Each iter an hour
            BigDecimal sec = BigDecimal.valueOf(3600);
            Vector acc = metric.getAcceleration(earth);
            Vector accNewton = newton.getPosition().unit().mul(-1).mul(Constants.G * mass / newton.getPosition().length2());

            earth.addVelocity(acc.mul(sec));
            earth.update(sec);

            newton.addVelocity(accNewton.mul(sec));
            newton.update(sec);

            System.out.println(CoordinateSystem.POLAR.toCartesianVelocity(earth.getPosition(), earth.getVelocity()));
            System.out.println(newton.getVelocity());
            System.out.println();
        }

        Vector end = earth.getPosition();
    }
}

import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.physics.metric.Schwarzschild;

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

        Vector start = earth.getPosition();
        System.out.println(start);
        System.out.println();

        for (int i=0;i<87600;i++) { // Each iter an hour
            BigDecimal sec = BigDecimal.valueOf(3600);
            Vector acc = metric.getAcceleration(earth);
            Vector accNewton = newton.getPosition().unit().mul(BigDecimal.ONE.negate()).mul(Constants.G.multiply(mass).divide(newton.getPosition().length2(), MathContext.DECIMAL128));

            earth.addVelocity(acc.mul(sec));
            earth.update(sec);

            newton.addVelocity(accNewton.mul(sec));
            newton.update(sec);

            System.out.println(i * 100d / 87600);
        }

        System.out.println();
        System.out.println(CoordinateSystem.POLAR.toCartesianVelocity(earth.getPosition(), earth.getVelocity()).round(MathContext.DECIMAL64));
        System.out.println(newton.getVelocity().round(MathContext.DECIMAL64));
    }
}

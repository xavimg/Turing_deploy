import org.junit.Assert;
import org.junit.Test;
import org.proj.math.Rand;
import org.proj.math.Range;
import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.metric.Kerr;

public class KerrTest {
    Kerr earth = new Kerr(3e-6, 0.02128, 7.292115e-5); // EARTH

    @Test
    public void coordinateTest() {
        Range.ofInt(0, Short.MAX_VALUE, true).forEach((i) -> {
            Vector pos = Vector.of(Rand.nextDouble(-1000, 1000), Rand.nextDouble(-1000, 1000));
            Vector kerr = earth.fromCartesianPosition(pos);
            Vector back = earth.toCartesianPosition(kerr);

            Assert.assertEquals(pos, back);
        });
    }

    @Test
    public void positionTest() {
        Range.ofInt(0, Short.MAX_VALUE, true).forEach(i -> {
            Vector pos = Vector.of(Rand.nextDouble(-1000, 1000), Rand.nextDouble(-1000, 1000));
            Vector vel = Vector.of(Rand.nextDouble(-1000, 1000), Rand.nextDouble(-1000, 1000));

            Vector kerrPos = earth.fromCartesianPosition(pos);
            Vector kerr = earth.fromCartesianVelocity(pos, vel);
            Vector back = earth.toCartesianVelocity(kerrPos, kerr);

            Vector dist = vel.subtr(back);
            System.out.println(dist.length());
        });
    }
}

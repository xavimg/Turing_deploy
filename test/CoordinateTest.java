import org.proj.math.Rand;
import org.proj.math.vector.Vector;
import org.proj.physics.coordinate.CoordinateSystem;

public class CoordinateTest {
    public static void main (String... main) {
        Vector position = Vector.of(Rand.nextDouble(-1, 1), Rand.nextDouble(-1, 1));
        Vector velocity = Vector.of(Rand.nextDouble(-10, 10), Rand.nextDouble(-10, 10));

        Vector polarPosition = CoordinateSystem.POLAR.fromCartesianPosition(position);
        Vector polarVelocity = CoordinateSystem.POLAR.fromCartesianVelocity(position, velocity);

        System.out.println(velocity);
        System.out.println(polarVelocity);
        System.out.println(CoordinateSystem.POLAR.toCartesianVelocity(polarPosition, polarVelocity));
    }
}

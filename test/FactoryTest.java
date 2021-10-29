import org.junit.Test;
import org.proj.game.Planet;
import org.proj.game.PlanetarySystem;
import org.proj.game.Sun;
import org.proj.game.factory.PlanetFactory;
import org.proj.game.factory.PlanetarySystemFactory;
import org.proj.game.factory.SunFactory;
import org.proj.math.MathUtils;
import org.proj.math.Range;

import java.util.Comparator;
import java.util.List;
import java.util.function.DoubleUnaryOperator;
import java.util.stream.Collectors;

public class FactoryTest {
    @Test
    public void testPlanet() {
        PlanetFactory factory = new PlanetFactory();
        Planet planet = factory.get();
        System.out.println();
    }

    @Test
    public void testSun() {
        SunFactory factory = new SunFactory();

        List<Sun> suns = Range.ofInt(0, 20, true).mapToObj(i -> factory.get()).sorted(Comparator.comparingDouble(x -> x.temperature)).collect(Collectors.toList());
        System.out.println();
    }

    @Test
    public void testSystem() {
        PlanetarySystemFactory factory = new PlanetarySystemFactory();
        PlanetarySystem system = factory.get();
        System.out.println();
    }
}

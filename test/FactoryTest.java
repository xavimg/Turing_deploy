import org.junit.Test;
import org.proj.game.Planet;
import org.proj.game.factory.PlanetFactory;

public class FactoryTest {
    @Test
    public void testPlanet() {
        PlanetFactory factory = new PlanetFactory();
        Planet planet = factory.get();
        System.out.println();
    }
}

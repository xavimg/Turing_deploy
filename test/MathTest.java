import org.junit.Test;
import org.proj.math.numbers.Complex;
import org.proj.math.numbers.Fraction;
import org.proj.math.vector.Vector;

public class MathTest {
    @Test
    public void fraction() {
        Fraction alpha = Fraction.valueOf(Math.PI);
        System.out.println(alpha.doubleValue());
    }
}

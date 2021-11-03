import org.junit.Test;
import org.proj.math.MathComplex;
import org.proj.math.MathUtils;
import org.proj.math.Range;
import org.proj.math.numbers.Complex;
import org.proj.math.numbers.Fraction;
import org.proj.math.vector.Vector;

import java.util.function.DoubleBinaryOperator;
import java.util.function.DoubleUnaryOperator;

public class MathTest {
    @Test
    public void fraction() {
        Fraction alpha = Fraction.valueOf(Math.PI);
        System.out.println(alpha.doubleValue());
    }
}

import org.proj.math.Cordic;

import java.math.BigDecimal;

public class CordicTest {
    public static void main (String... args) {
        var test = Cordic.tan(BigDecimal.valueOf(23));
        System.out.println();
    }
}

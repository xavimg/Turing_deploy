package garbagio;

import org.proj.math.big.Cordic;

import java.math.BigDecimal;

public class CordicTest {
    public static void main (String... args) {
        var test = Cordic.tan(BigDecimal.valueOf(23));
        System.out.println();
    }
}

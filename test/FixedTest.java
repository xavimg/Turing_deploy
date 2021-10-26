import org.junit.Assert;
import org.junit.Test;
import org.proj.math.Rand;
import org.proj.math.Range;
import org.proj.math.numbers.Chungus;
import org.proj.math.numbers.ULong;
import org.proj.utils.Compare;

import java.math.BigInteger;
import java.util.function.BinaryOperator;
import java.util.function.Function;

public class FixedTest {
    @Test
    public void testAdd() {
        Range.parallelOfInt(0, Short.MAX_VALUE).forEach(i -> {
            Chungus alpha = Rand.nextChungus().shiftRight(2);
            Chungus beta = Rand.nextChungus().shiftRight(2);

            BigInteger Alpha = alpha.bigIntegerValue();
            BigInteger Beta = beta.bigIntegerValue();

            Chungus gamma = alpha.add(beta);
            BigInteger Gamma = Alpha.add(Beta);

            Assert.assertEquals(Gamma.toString(), gamma.toString());
        });
    }

    @Test
    public void testSubtr() {
        Range.parallelOfInt(0, Short.MAX_VALUE).forEach(i -> {
            Chungus alpha = Rand.nextChungus().shiftRight(2);
            Chungus beta = Rand.nextChungus().shiftRight(2);

            BigInteger Alpha = alpha.bigIntegerValue();
            BigInteger Beta = beta.bigIntegerValue();

            Chungus gamma = alpha.subtr(beta);
            BigInteger Gamma = Alpha.subtract(Beta);

            Assert.assertEquals(Gamma.toString(), gamma.toString());
        });
    }

    @Test
    public void testMul() {
        Range.parallelOfInt(0, Short.MAX_VALUE).forEach(i -> {
            Chungus alpha = Rand.nextChungus().shiftRight(65);
            Chungus beta = Rand.nextChungus().shiftRight(65);

            BigInteger Alpha = alpha.bigIntegerValue();
            BigInteger Beta = beta.bigIntegerValue();

            Chungus gamma = alpha.mul(beta);
            BigInteger Gamma = Alpha.multiply(Beta);

            Assert.assertEquals(Gamma.toString(), gamma.toString());
        });
    }

    @Test
    public void testDiv () {
        Range.parallelOfInt(0, Short.MAX_VALUE).forEach(i -> {
            Chungus alpha = Rand.nextChungus().shiftRight(32);
            Chungus beta = Rand.nextChungus().shiftRight(64);

            BigInteger Alpha = alpha.bigIntegerValue();
            BigInteger Beta = beta.bigIntegerValue();

            Chungus gamma = alpha.subtr(beta);
            BigInteger Gamma = Alpha.subtract(Beta);

            Assert.assertEquals(Gamma.toString(), gamma.toString());
        });
    }
}

package org.proj.math;

import org.proj.utils.Compare;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.math.MathContext;
import java.math.RoundingMode;

public class MathUtils {
    final public static BigDecimal TWO = BigDecimal.valueOf(2);
    final public static BigDecimal THREE = BigDecimal.valueOf(3);
    final public static BigDecimal FOUR = BigDecimal.valueOf(4);
    final public static BigDecimal PI = new BigDecimal("3.1415926535897932384626433832795028841971693993751058209749445923");

    final public static BigDecimal PI_2 = TWO.multiply(PI);
    final public static BigDecimal PI2 = PI.pow(2);
    final public static BigDecimal HALF_PI = PI.divide(TWO);

    final public static MathContext DECIMAL256 = new MathContext(71, RoundingMode.HALF_EVEN);
    final public static BigDecimal LIMIT = new BigDecimal(BigInteger.ONE, 34);

    public static BigDecimal hypot (BigDecimal alpha, BigDecimal beta) {
        return alpha.pow(2).add(beta.pow(2)).sqrt(MathContext.DECIMAL128);
    }

    public static BigDecimal atan (BigDecimal value) {
        value = value.divide(BigDecimal.ONE.add(BigDecimal.ONE.add(value.pow(2)).sqrt(DECIMAL256)), DECIMAL256);
        value = value.divide(BigDecimal.ONE.add(BigDecimal.ONE.add(value.pow(2)).sqrt(DECIMAL256)), DECIMAL256);

        BigDecimal value2 = value.pow(2);
        BigDecimal result = value;

        BigDecimal k = MathUtils.THREE;
        boolean add = false;
        BigDecimal pow = value;

        while (true) {
            pow = pow.multiply(value2);
            BigDecimal delta = pow.divide(k, DECIMAL256);

            if (delta.compareTo(LIMIT) <= 0) {
                return result.multiply(MathUtils.FOUR, MathContext.DECIMAL128);
            }

            result = add ? result.add(delta) : result.subtract(delta);
            k = k.add(MathUtils.TWO);
            add = !add;
        }
    }

    public static BigDecimal atan2 (BigDecimal beta, BigDecimal alpha) {
        int compAlpha = alpha.compareTo(BigDecimal.ZERO);
        int compBeta = beta.compareTo(BigDecimal.ZERO);

        if (compBeta == 0 && compAlpha == 0) {
            return null;
        }

        BigDecimal atan = atan(beta.divide(alpha, DECIMAL256));
        if (compAlpha > 0) {
            return atan;
        } else if (compBeta > 0) {
            return HALF_PI.subtract(atan);
        } else if (compBeta < 0) {
            return HALF_PI.add(atan).negate();
        }

        return atan.add(PI);
    }
}

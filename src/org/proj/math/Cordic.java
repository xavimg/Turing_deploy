package org.proj.math;

import org.proj.math.vector.Vector;
import org.proj.utils.Compare;

import java.math.BigDecimal;
import java.math.MathContext;
import java.util.ArrayList;

public class Cordic {
    final private static ArrayList<BigDecimal> angles = new ArrayList<>();
    final private static ArrayList<BigDecimal> AValues = new ArrayList<>();

    static {
        angles.add(MathUtils.PI.divide(MathUtils.FOUR, MathContext.DECIMAL128));
        AValues.add(MathUtils.TWO.sqrt(MathContext.DECIMAL128));
    }

    private static BigDecimal getAngle (int index) {
        if (angles.size() <= index) {
            int k = angles.size() - 1;
            BigDecimal value = BigDecimal.ONE.divide(MathUtils.TWO.pow(k), MathUtils.DECIMAL256);

            for (int i=k;i<=index;i++) {
                value = value.divide(MathUtils.TWO);
                angles.add(MathUtils.atan(value));
            }
        }

        return angles.get(index);
    }

    private static BigDecimal getAValue (int index) {
        if (AValues.size() <= index) {
            int j = AValues.size() - 1;

            BigDecimal k = AValues.get(j);
            BigDecimal value = BigDecimal.ONE.divide(MathUtils.TWO.pow(-2 * j), MathUtils.DECIMAL256);

            for (int i=j;i<=index;i++) {
                value = value.divide(MathUtils.FOUR, MathUtils.DECIMAL256);
                BigDecimal delta = BigDecimal.ONE.add(value).sqrt(MathContext.DECIMAL128);
                k = k.multiply(delta);
                AValues.add(k);
            }
        }

        return AValues.get(index);
    }

    private static BigDecimal getKValue (int index) {
        return BigDecimal.ONE.divide(getAValue(index), MathContext.DECIMAL128);
    }

    public static Vector cosSin(BigDecimal value) {
        if (Compare.isLesser(value, MathUtils.HALF_PI.negate())) {
            return cosSin(value.add(MathUtils.PI)).mul(BigDecimal.ONE.negate());
        } else if (Compare.isGreater(value, MathUtils.HALF_PI)) {
            return cosSin(value.subtract(MathUtils.PI)).mul(BigDecimal.ONE.negate());
        }

        Vector v = Vector.of(BigDecimal.ONE, BigDecimal.ZERO);
        BigDecimal pow2 = BigDecimal.ONE;
        BigDecimal angle;

        int i = 0;
        while (true) {
            angle = getAngle(i);
            BigDecimal sigma = Compare.isLesser(value, BigDecimal.ZERO) ? BigDecimal.ONE.negate() : BigDecimal.ONE;

            BigDecimal x = v.get(0).subtract(sigma.multiply(v.get(1).multiply(pow2)));
            BigDecimal y = sigma.multiply(v.get(0).multiply(pow2)).add(v.get(1));

            BigDecimal dx = x.subtract(v.get(0)).abs();
            BigDecimal dy = y.subtract(v.get(1)).abs();

            if (Compare.isLesserOrEqual(dx, MathUtils.LIMIT) && Compare.isLesserOrEqual(dy, MathUtils.LIMIT)) {
                return v.mul(getKValue(i)).round(MathContext.DECIMAL128);
            }

            v = Vector.of(x, y);
            value = value.subtract(sigma.multiply(angle));
            pow2 = pow2.divide(MathUtils.TWO);
            i++;
        }
    }

    public static BigDecimal sin (BigDecimal value) {
        return cosSin(value).get(1);
    }

    public static BigDecimal cos (BigDecimal value) {
        return cosSin(value).get(0);
    }

    public static BigDecimal tan (BigDecimal value) {
        Vector cosSin = cosSin(value);
        return cosSin.get(1).divide(cosSin.get(0), MathContext.DECIMAL128);
    }
}

package org.proj.physics.coordinate;

import org.proj.math.Cordic;
import org.proj.math.MathUtils;
import org.proj.math.vector.Vector;

import java.math.BigDecimal;
import java.math.MathContext;

public class Polar implements CoordinateSystem {
    protected Polar () {};

    @Override
    public Vector fromCartesianPosition (Vector cartesian) {
        BigDecimal x = cartesian.get(0);
        BigDecimal y = cartesian.get(1);

        return Vector.of(MathUtils.hypot(x, y), MathUtils.atan2(y, x));
    }

    @Override
    public Vector toCartesianPosition (Vector position) {
        BigDecimal r = position.get(0);
        BigDecimal theta = position.get(1);

        return Cordic.cosSin(theta).mul(r);
    }

    @Override
    public Vector fromCartesianVelocity (Vector position, Vector cartesian) {
        BigDecimal x = position.get(0);
        BigDecimal y = position.get(1);

        BigDecimal vx = cartesian.get(0);
        BigDecimal vy = cartesian.get(1);

        BigDecimal r2 = x.pow(2).add(y.pow(2));
        BigDecimal r = r2.sqrt(MathContext.DECIMAL128);

        return Vector.of(x.multiply(vx).add(y.multiply(vy)).divide(r, MathContext.DECIMAL128), x.multiply(vy).subtract(y.multiply(vx)).divide(r2, MathContext.DECIMAL128));
    }

    @Override
    public Vector toCartesianVelocity (Vector position, Vector velocity) {
        BigDecimal r = position.get(0);
        BigDecimal theta = position.get(1);

        Vector cosSin = Cordic.cosSin(theta);
        BigDecimal sin = cosSin.get(1);
        BigDecimal cos = cosSin.get(0);

        BigDecimal vr = velocity.get(0);
        BigDecimal vtheta = velocity.get(1);

        BigDecimal lambda = r.multiply(vtheta);
        return Vector.of(vr.multiply(cos).subtract(lambda.multiply(sin)), vr.multiply(sin).add(lambda.multiply(cos)));
    }
}

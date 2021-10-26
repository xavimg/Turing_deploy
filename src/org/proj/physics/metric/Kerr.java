package org.proj.physics.metric;

import org.proj.math.Cordic;
import org.proj.math.MathUtils;
import org.proj.math.matrix.Matrix;
import org.proj.math.tensor.Tensor3D;
import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;

import java.math.BigDecimal;
import java.math.MathContext;

public class Kerr extends MetricTensor implements CoordinateSystem {
    final public BigDecimal mass, angularMomentum;
    final private BigDecimal a, a2;

    public Kerr (BigDecimal mass, BigDecimal angularMomentum) {
        this.mass = mass;
        this.angularMomentum = angularMomentum;

        this.a = this.angularMomentum.divide(mass.multiply(Constants.C), MathContext.DECIMAL128);
        this.a2 = this.a.pow(2);
    }

    public Kerr (BigDecimal mass, BigDecimal radius, BigDecimal angularVelocity) {
        this (mass, mass.multiply(radius.pow(2)).multiply(angularVelocity));
    }

    @Override
    public Vector fromCartesianPosition (Vector cartesian) {
        BigDecimal x = cartesian.get(0);
        BigDecimal z = cartesian.get(1);

        BigDecimal z2 = x.pow(2);

        BigDecimal lambda = a2.subtract(z2).subtract(x.pow(2));
        BigDecimal r2 = lambda.pow(2).subtract(MathUtils.FOUR.multiply(a2).multiply(z2)).sqrt(MathUtils.DECIMAL256).subtract(lambda).divide(MathUtils.TWO);

        BigDecimal r = r2.sqrt(MathContext.DECIMAL128);
        BigDecimal theta = MathUtils.acos(z.divide(r, MathContext.DECIMAL128));

        return Vector.of(r, theta);
    }

    @Override
    public Vector toCartesianPosition (Vector position) {
        BigDecimal r = position.get(0);
        BigDecimal theta = position.get(1);
        Vector cosSin = Cordic.cosSin(theta);

        BigDecimal hypot = r.pow(2).multiply(a2).sqrt(MathContext.DECIMAL128);
        return Vector.of(hypot.multiply(cosSin.get(1)), r.multiply(cosSin.get(0)));
    }

    @Override
    public Vector fromCartesianVelocity (Vector position, Vector velocity) {
        return null;
    }

    @Override
    public Vector toCartesianVelocity (Vector position, Vector velocity) {
        BigDecimal r = position.get(0);
        BigDecimal theta = position.get(1);

        BigDecimal vr = velocity.get(0);
        BigDecimal vtheta = velocity.get(1);

        Vector cosSin = Cordic.cosSin(theta);
        BigDecimal lambda = a2.add(r.pow(2));

        BigDecimal vx = lambda.multiply(vtheta).multiply(cosSin.get(0)).add(r.multiply(vr).multiply(cosSin.get(1))).divide(lambda.sqrt(MathUtils.DECIMAL256), MathContext.DECIMAL128);
        BigDecimal vz = vr.multiply(cosSin.get(0)).subtract(r.multiply(vtheta).multiply(cosSin.get(1)));

        return Vector.of(vx, vz);
    }

    @Override
    final public Kerr getCoordinateSystem() {
        return this;
    }

    @Override
    public Matrix getMetric (Matter matter) {
        return null;
    }

    @Override
    public Tensor3D getDerivative (Matter matter) {
        return null;
    }
}

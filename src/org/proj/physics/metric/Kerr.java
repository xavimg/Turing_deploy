package org.proj.physics.metric;

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
    final public double mass, angularMomentum;
    final private double a, a2;

    public Kerr (double mass, double angularMomentum) {
        this.mass = mass;
        this.angularMomentum = angularMomentum;

        this.a = this.angularMomentum / (mass * Constants.C);
        this.a2 = this.a * this.a;
    }

    public Kerr (double mass, double radius, double angularVelocity) {
        this (mass, mass * radius * radius * angularVelocity);
    }

    @Override
    public Vector fromCartesianPosition(Vector cartesian) {
        return null;
    }

    @Override
    public Vector toCartesianPosition(Vector position) {
        return null;
    }

    @Override
    public Vector fromCartesianVelocity(Vector position, Vector velocity) {
        return null;
    }

    @Override
    public Vector toCartesianVelocity(Vector position, Vector velocity) {
        return null;
    }

    @Override
    public CoordinateSystem getCoordinateSystem() {
        return null;
    }

    @Override
    public Matrix getMetric(Matter matter) {
        return null;
    }

    @Override
    public Tensor3D getDerivative(Matter matter) {
        return null;
    }
}

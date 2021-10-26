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
    public Vector fromCartesianPosition (Vector cartesian) {
        double x = cartesian.get(0);
        double z = cartesian.get(1);

        double x2 = x * x;
        double z2 = z * z;

        double lambda = a2 - z2 - x2;
        double r2 = (Math.sqrt(lambda * lambda - 4 * a2 * z2) - lambda) / 2d;

        double r = Math.sqrt(r2);
        double theta = Math.acos(z / r);

        return Vector.of(r, theta);
    }

    @Override
    public Vector toCartesianPosition (Vector position) {
        double r = position.get(0);
        double theta = position.get(1);

        return Vector.of(Math.hypot(a, r) * Math.sin(theta), r * Math.cos(theta));
    }

    @Override
    public Vector fromCartesianVelocity (Vector position, Vector velocity) {
        double x = position.get(0);
        double z = position.get(1);

        double x2 = x * x;
        double z2 = z * z;

        double lambda = a2 - z2 - x2;
        double r2 = (Math.sqrt(lambda * lambda - 4 * a2 * z2) - lambda) / 2d;

        double r = Math.sqrt(r2);
        double cos = z / r;
        double theta = Math.acos(cos);

        double sin = Math.sin(theta);
        double tan = sin / cos;

        double vx = velocity.get(0);
        double vy = velocity.get(1);

        lambda = a2 + r;
        double lambdaSqrt = Math.sqrt(lambda);
        double psi = lambda * cos / lambdaSqrt;
        double lambda_2 = 2 * lambdaSqrt;

        double vtheta = (lambda_2 * vx - vy * tan) / (lambda_2 * psi + r * tan * sin);
        double vr = (vy / cos) + (r * vtheta * tan);

        return Vector.of(vr, vtheta);
    }

    @Override
    public Vector toCartesianVelocity (Vector position, Vector velocity) {
        double r = position.get(0);
        double theta = position.get(1);

        double sin = Math.sin(theta);
        double cos = Math.cos(theta);

        double vr = velocity.get(0);
        double vtheta = velocity.get(1);

        double lambda = a2 + r;
        double vx = (2 * lambda * vtheta * cos + vr * sin) / (2 * Math.sqrt(lambda));
        double vy = vr * cos - r * vtheta * sin;

        return Vector.of(vx, vy);
    }

    @Override
    final public CoordinateSystem getCoordinateSystem() {
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

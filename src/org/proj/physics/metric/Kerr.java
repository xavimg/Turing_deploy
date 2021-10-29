package org.proj.physics.metric;

import org.proj.math.MathUtils;
import org.proj.math.matrix.Matrix;
import org.proj.math.matrix.special.DiagonalMatrix;
import org.proj.math.matrix.special.ZeroMatrix;
import org.proj.math.tensor.LazyTensor3D;
import org.proj.math.tensor.Tensor3D;
import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.utils.Couple;

public class Kerr extends MetricTensor implements CoordinateSystem {
    final public double mass, angularMomentum;
    final private double a, a2, rs;

    public Kerr (double mass, double angularMomentum) {
        this.mass = mass;
        this.angularMomentum = angularMomentum;

        this.a = this.angularMomentum / (mass * Constants.C);
        this.rs = Schwarzschild.radius(mass);
        this.a2 = this.a * this.a;
    }

    public Kerr (double mass, double radius, double angularVelocity) {
        this (mass, mass * radius * radius * angularVelocity);
    }

    @Override
    public Couple<? extends Matrix, ? extends Tensor3D> calculateMetric (Matter matter) {
        double r = matter.getPosition().get(0);
        double theta = matter.getPosition().get(1);

        double sin = Math.sin(theta);
        double cos = Math.cos(theta);
        double r2 = r * r;
        double cos2 = cos * cos;

        double lambda = a2 * cos2;
        double sigma = r2 + lambda;
        double delta = r2 - rs * r + a2;

        double a2cos2 = a2 * cos2;
        double a2sincos = a2 * sin * cos;
        double sigma2 = sigma * sigma;

        DiagonalMatrix metric = new DiagonalMatrix(
                Constants.C2 * (1 - (rs * r / sigma)),
                -sigma / delta,
                -sigma
        );

        LazyTensor3D deriv = new LazyTensor3D.OfMatrix(3, 3, 3) {
            @Override
            public Matrix compute (int i) {
                return switch (i) {
                    case 1 -> new DiagonalMatrix(
                            Constants.C2 * rs * (r2 - lambda) / sigma2,
                            (a2cos2 * (2 * r - rs) + r * (r * rs - 2 * a2)) / Math.pow(a2 + r * (r - rs), 2),
                            -2 * r

                    );

                    case 2 -> new DiagonalMatrix(
                        2 * a2sincos * rs * r * Constants.C2 / sigma2,
                            2 * a2sincos/ delta,
                            2 * a2sincos
                    );

                    default -> new ZeroMatrix(3, 3);
                };
            }
        };

        return new Couple<>(metric, deriv);
    }

    @Override
    public Matrix getMetric (Matter matter) {
        double r = matter.getPosition().get(0);
        double theta = matter.getPosition().get(1);

        double cos = Math.cos(theta);
        double r2 = r * r;

        double lambda = a2 * Math.pow(cos, 2);
        double sigma = r2 + lambda;
        double delta = r2 - rs * r + a2;

        return new DiagonalMatrix(
                Constants.C2 * (1 - rs * r / sigma),
                -sigma / delta,
                -sigma
        );
    }

    @Override
    public Tensor3D getDerivative (Matter matter) {
        double r = matter.getPosition().get(0);
        double theta = matter.getPosition().get(1);

        double sin = Math.sin(theta);
        double cos = Math.cos(theta);
        double r2 = r * r;

        double lambda = a2 * Math.pow(cos, 2);
        double sigma = r2 + lambda;
        double delta = r2 - rs * r + a2;

        double a2cos = a2 * cos;
        double a2sin = a2 * sin;
        double sigma2 = sigma * sigma;

        return new LazyTensor3D.OfMatrix(3, 3, 3) {
            @Override
            public Matrix compute (int i) {
                return switch (i) {
                    case 1 -> new DiagonalMatrix(
                            Constants.C2 * rs * (r2 - lambda) / sigma2,
                            (a2cos * (2 * r - rs) + r * (r * rs - 2 * a2)) / Math.pow(a2 + r * (r - rs), 2),
                            -2 * r

                    );

                    case 2 -> new DiagonalMatrix(
                            2 * a2cos * rs * r * sin * Constants.C2 / sigma2,
                            a2sin / delta,
                            a2sin
                    );

                    default -> new ZeroMatrix(3, 3);
                };
            }
        };
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

    public Vector fromKerrPosAndCartesianVelocity (Vector kerrPos, Vector velocity) {
        double r = kerrPos.get(0);
        double theta = kerrPos.get(1);

        double sin = Math.sin(theta);
        double cos = Math.cos(theta);
        double tan = sin / cos;

        double vx = velocity.get(0);
        double vy = velocity.get(1);

        double lambda = a2 + r;
        double lambdaSqrt = Math.sqrt(lambda);
        double psi = lambda * cos / lambdaSqrt;
        double lambda_2 = 2 * lambdaSqrt;

        double vtheta = (lambda_2 * vx - vy * tan) / (lambda_2 * psi + r * tan * sin);
        double vr = (vy / cos) + (r * vtheta * tan);

        return Vector.of(vr, vtheta);
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
}

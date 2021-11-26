package org.proj.physics.metric;

import org.proj.math.matrix.Mat3;
import org.proj.math.Tens3;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.utils.Couple;

public class Schwarzschild extends MetricTensor {
    final public double mass;
    final private double isco;
    final private double value;

    public Schwarzschild (double mass) {
        this.mass = mass;
        this.value = 2 * Constants.G * mass;
        this.isco = 6 * Constants.G * mass / Constants.C2;
    }

    @Override
    public double getIsco (Matter matter) {
        return isco;
    }

    @Override
    public CoordinateSystem getCoordinateSystem() {
        return CoordinateSystem.POLAR;
    }

    @Override
    public Couple<Mat3, Tens3> calculateMetric (Matter matter) {
        double r = matter.getPosition().x;
        double _alpha = value / r;

        Mat3 metric = new Mat3(
                Constants.C2 - _alpha,
                -1d / (1 - (_alpha / Constants.C2)),
                -(r * r)
        );

        Tens3 deriv = new Tens3(
                Mat3.ZERO,
                new Mat3(_alpha / r, value * Constants.C2 / Math.pow(Constants.C2 * r - _alpha, 2), -2 * r),
                Mat3.ZERO
        );

        return new Couple<>(metric, deriv);
    }

    @Override
    public Mat3 getMetric (Matter matter) {
        double r = matter.getPosition().x;
        double _alpha = value / r;

        return new Mat3(
                Constants.C2 - _alpha,
                -1d / (1 - (_alpha / Constants.C2)),
                -(r * r)
        );
    }

    @Override
    public Tens3 getDerivative (Matter matter) {
        double r = matter.getPosition().x;
        double _alpha = value / r;

        Tens3 deriv = new Tens3(
                Mat3.ZERO,
                new Mat3(_alpha / r, value * Constants.C2 / Math.pow(Constants.C2 * r - _alpha, 2), -2 * r),
                Mat3.ZERO
        );

        return deriv;
    }

    public static double radius (double mass) {
        return 2 * Constants.G * mass / Constants.C2;
    }

    public static double mass (double radius) {
        return radius * Constants.C2 / (2 * Constants.G);
    }
}

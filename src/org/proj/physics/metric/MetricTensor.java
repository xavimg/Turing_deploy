package org.proj.physics.metric;

import org.proj.math.MathUtils;
import org.proj.math.Tens3;
import org.proj.math.Mat3;
import org.proj.math.vector.Vec2;
import org.proj.math.vector.Vec3;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.utils.Couple;

public abstract class MetricTensor {
    public abstract CoordinateSystem getCoordinateSystem ();
    public abstract double getIsco (Matter matter);
    public abstract Mat3 getMetric (Matter matter);
    public abstract Tens3 getDerivative (Matter matter);

    public Couple<Mat3, Tens3> calculateMetric (Matter matter) {
        return new Couple<>(getMetric(matter), getDerivative(matter));
    }

    /**
     * Calculates time dilation for a diagonal metric tensor
     * @param metric Metric tensor
     * @param matter Matter
     * @return Time dilation
     */
    public double getTimeDilation (Mat3 metric, Matter matter) {
        double v1 = matter.getVelocity().x;
        double v2 = matter.getVelocity().y;

        double a = metric.x.x;
        double b = v1 * (metric.x.y + metric.y.x) + v2 * (metric.x.z + metric.z.x);
        double c = v1 * v2 * (metric.y.y + metric.y.z + metric.z.y + metric.z.z) - Constants.C2;

        return (-b + Math.sqrt(b * b - 4 * a * c)) / (2 * a);
    }

    /**
     * @param metric Metric tensor
     * @param deriv Metric tensor's derivative over each position
     * @return The calculated Christoffel Symbols
     */
    final public Tens3 getChristoffel (Mat3 metric, Tens3 deriv) {
        Mat3 inverse = metric.inverse();

        return Tens3.of((i,j,k) -> MathUtils.sum(3, (int q) -> {
            double sum = deriv.get(k, q, j) + deriv.get(j, q, k) - deriv.get(q, j, k);
            return inverse.get(i, q) * sum / 2d;
        }));
    }

    final public Tens3 getChristoffel (Matter matter) {
        Couple<Mat3, Tens3> calc = calculateMetric(matter);
        return getChristoffel(calc.first, calc.last);
    }

    /**
     * @param christoffel Christoffel Symbols
     * @param vt Time dilation
     * @param vel Space velocity
     * @see #getChristoffel(Mat3, Tens3)
     * @return Acceleration given as dv / d&tau;
     */
    final public Vec2 getProperAcceleration (Tens3 christoffel, double vt, Vec2 vel) {
        Vec3 velocity = new Vec3(vt, vel);
        double x = -MathUtils.sum(3, (int i) -> MathUtils.sum(3, (int j) -> christoffel.get(1, i, j) * velocity.get(i) * velocity.get(j)));
        double y = -MathUtils.sum(3, (int i) -> MathUtils.sum(3, (int j) -> christoffel.get(2, i, j) * velocity.get(i) * velocity.get(j)));

        return new Vec2(x, y);
    }

    final public Vec2 getAcceleration (Matter matter) {
        Couple<Mat3, Tens3> calc = calculateMetric(matter);
        Tens3 christoffel = getChristoffel(calc.first, calc.last);
        double timeDilation = this.getTimeDilation(calc.first, matter);

        Vec2 properAcc = getProperAcceleration(christoffel, timeDilation, matter.getVelocity());
        return properAcc.div(timeDilation);
    }
}

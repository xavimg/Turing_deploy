package org.proj.physics.metric;

import org.proj.math.MathUtils;
import org.proj.math.matrix.Matrix;
import org.proj.math.matrix.special.DiagonalMatrix;
import org.proj.math.tensor.LazyTensor3D;
import org.proj.math.tensor.Tensor3D;
import org.proj.math.vector.LazyVector;
import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.utils.Couple;

import java.math.BigDecimal;
import java.math.MathContext;

public abstract class MetricTensor {
    final private static BigDecimal TWO = BigDecimal.valueOf(2);
    public abstract CoordinateSystem getCoordinateSystem ();

    public abstract Matrix getMetric (Matter matter);
    public abstract Tensor3D getDerivative (Matter matter);

    public Couple<? extends Matrix, ? extends Tensor3D> calculateMetric (Matter matter) {
        return new Couple<>(getMetric(matter), getDerivative(matter));
    }

    /**
     * Calculates time dilation for a diagonal metric tensor
     * @param metric Metric tensor
     * @param matter Matter
     * @return Time dilation
     */
    public BigDecimal getTimeDilation (DiagonalMatrix metric, Matter matter) {
        Vector vector = metric.getVector();
        Vector vel = matter.getVelocity();

        BigDecimal sum = vector.get(1).multiply(vel.get(0).pow(2));
        sum = sum.add(vector.get(2).multiply(vel.get(1).pow(2)));

        return Constants.C2.subtract(sum).divide(vector.get(0), MathContext.DECIMAL128).sqrt(MathContext.DECIMAL128);
    }

    /**
     * @param metric Metric tensor
     * @param deriv Metric tensor's derivative over each position
     * @return The calculated Christoffel Symbols
     */
    final public LazyTensor3D getChristoffel (Matrix metric, Tensor3D deriv) {
        return new LazyTensor3D (3, 3, 3) {
            Matrix inverse = metric.inverse();

            @Override
            public BigDecimal compute (int i, int j, int k) {
                return MathUtils.sum(3, (int q) -> {
                    BigDecimal sum = deriv.get(k, q, j).add(deriv.get(j, q, k)).subtract(deriv.get(q, j, k));
                    return inverse.get(i, q).multiply(sum).divide(TWO);
                });
            }
        };
    }

    /**
     * @param christoffel Christoffel Symbols
     * @param vt Time dilation
     * @param vel Space velocity
     * @see #getChristoffel(Matrix, Tensor3D)
     * @return Acceleration given as dv / d&tau;
     */
    final public Vector getProperAcceleration (Tensor3D christoffel, BigDecimal vt, Vector vel) {
        return new LazyVector (3) {
            final Vector velocity = new Vector(3) {
                @Override
                public BigDecimal get(int i) {
                    return i == 0 ? vt : vel.get(i - 1);
                }
            };

            @Override
            public BigDecimal compute (int pos) {
                return MathUtils.sum(3, (int i) -> MathUtils.sum(3, (int j) -> christoffel.get(pos, i, j).multiply(velocity.get(i)).multiply(velocity.get(j)))).negate();
            }
        }.copyOf(1);
    }

    final public Vector getAcceleration (Matter matter) {
        Couple<? extends Matrix, ? extends Tensor3D> calc = calculateMetric(matter);
        Tensor3D christoffel = getChristoffel(calc.first, calc.last);

        BigDecimal timeDilation;
        if (calc.first instanceof DiagonalMatrix) {
            timeDilation = this.getTimeDilation((DiagonalMatrix) calc.first, matter);
        } else {
            return null;
        }

        Vector properAcc = getProperAcceleration(christoffel, timeDilation, matter.getVelocity());
        return properAcc.div(timeDilation);
    }
}

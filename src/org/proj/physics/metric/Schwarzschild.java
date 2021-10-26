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

import java.math.BigDecimal;
import java.math.MathContext;

public class Schwarzschild extends MetricTensor {
    final public double mass;
    final private double value;

    public Schwarzschild (double mass) {
        this.mass = mass;
        this.value = 2 * Constants.G * mass;
    }

    @Override
    public CoordinateSystem getCoordinateSystem() {
        return CoordinateSystem.POLAR;
    }

    @Override
    public Couple<DiagonalMatrix, Tensor3D> calculateMetric (Matter matter) {
        double r = matter.getPosition().get(0);
        double _alpha = value / r;

        DiagonalMatrix metric = new DiagonalMatrix(
                Constants.C2 - _alpha,
                -1d / (1 - (_alpha / Constants.C2)),
                -(r * r)
        );

        Tensor3D deriv = new LazyTensor3D.OfMatrix (3, 3, 3) {
            @Override
            public Matrix compute (int i) {
                return switch (i) {
                    case 1 -> new DiagonalMatrix(Vector.of(_alpha / r, value * Constants.C2 / Math.pow(Constants.C2 * r - _alpha, 2), -2 * r));
                    default -> new ZeroMatrix(3, 3);
                };
            }
        };

        return new Couple<>(metric, deriv);
    }

    @Override
    public DiagonalMatrix getMetric (Matter matter) {
        double r = matter.getPosition().get(0);
        double _alpha = value / r;

        DiagonalMatrix metric = new DiagonalMatrix(
                Constants.C2 - _alpha,
                -1d / (1 - (_alpha / Constants.C2)),
                -(r * r)
        );

        return metric;
    }

    @Override
    public Tensor3D getDerivative (Matter matter) {
        double r = matter.getPosition().get(0);
        double _alpha = value / r;

        Tensor3D deriv = new LazyTensor3D.OfMatrix (3, 3, 3) {
            @Override
            public Matrix compute (int i) {
                return switch (i) {
                    case 1 -> new DiagonalMatrix(Vector.of(_alpha / r, value * Constants.C2 / Math.pow(Constants.C2 * r - _alpha, 2), -2 * r));
                    default -> new ZeroMatrix(3, 3);
                };
            }
        };

        return deriv;
    }

    public static double radius (double mass) {
        return 2 * Constants.G * mass / Constants.C2;
    }
}

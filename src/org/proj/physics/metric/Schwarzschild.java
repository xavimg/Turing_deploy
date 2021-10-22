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
    final public BigDecimal mass;
    final private BigDecimal value;

    public Schwarzschild (BigDecimal mass) {
        this.mass = mass;
        this.value = MathUtils.TWO.multiply(Constants.G).multiply(mass);
    }

    @Override
    public CoordinateSystem getCoordinateSystem() {
        return CoordinateSystem.POLAR;
    }

    @Override
    public Couple<DiagonalMatrix, Tensor3D> calculateMetric (Matter matter) {
        BigDecimal r = matter.getPosition().get(0);
        BigDecimal _alpha = value.divide(r, MathContext.DECIMAL128);

        DiagonalMatrix metric = new DiagonalMatrix(
                Constants.C2.subtract(_alpha),
                BigDecimal.ONE.negate().divide(BigDecimal.ONE.subtract(_alpha.divide(Constants.C2, MathContext.DECIMAL128)), MathContext.DECIMAL128),
                r.pow(2).negate()
        );

        Tensor3D deriv = new LazyTensor3D.OfMatrix (3, 3, 3) {
            @Override
            public Matrix compute (int i) {
                return switch (i) {
                    case 1 -> new DiagonalMatrix(Vector.of(_alpha.divide(r, MathContext.DECIMAL128), value.multiply(Constants.C2).divide(Constants.C2.multiply(r).subtract(_alpha).pow(2), MathContext.DECIMAL128), r.negate().multiply(MathUtils.TWO)));
                    default -> new ZeroMatrix(3, 3);
                };
            }
        };

        return new Couple<>(metric, deriv);
    }

    @Override
    public DiagonalMatrix getMetric (Matter matter) {
        BigDecimal r = matter.getPosition().get(0);
        BigDecimal _alpha = value.divide(r);

        return new DiagonalMatrix(
                Constants.C2.subtract(_alpha),
                BigDecimal.ONE.negate().divide(BigDecimal.ONE.subtract(_alpha.divide(Constants.C2, MathContext.DECIMAL128)), MathContext.DECIMAL128),
                r.pow(2).negate()
        );
    }

    @Override
    public Tensor3D getDerivative (Matter matter) {
        BigDecimal r = matter.getPosition().get(0);
        BigDecimal _alpha = value.divide(r);

        return new LazyTensor3D.OfMatrix (3, 3, 3) {
            @Override
            public Matrix compute (int i) {
                return switch (i) {
                    case 1 -> new DiagonalMatrix(Vector.of(_alpha.divide(r, MathContext.DECIMAL128), value.multiply(Constants.C2).divide(Constants.C2.multiply(r).subtract(_alpha).pow(2), MathContext.DECIMAL128), r.negate().multiply(MathUtils.TWO)));
                    default -> new ZeroMatrix(3, 3);
                };
            }
        };
    }

    public static BigDecimal radius (BigDecimal mass) {
        return MathUtils.TWO.multiply(Constants.G).multiply(mass).divide(Constants.C2, MathContext.DECIMAL128);
    }
}

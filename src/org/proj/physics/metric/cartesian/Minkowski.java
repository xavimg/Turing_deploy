package org.proj.physics.metric.cartesian;

import org.proj.math.matrix.Matrix;
import org.proj.math.matrix.special.DiagonalMatrix;
import org.proj.math.tensor.Tensor3D;
import org.proj.math.tensor.ZeroTensor;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import org.proj.physics.metric.MetricTensor;

public class Minkowski extends MetricTensor {
    final public static Minkowski METRIC = new Minkowski();

    private Minkowski() {
        super(Double.POSITIVE_INFINITY);
    }

    @Override
    final public CoordinateSystem getCoordinateSystem() {
        return CoordinateSystem.CARTESIAN;
    }

    @Override
    public Matrix getMetric (Matter matter) {
        return new DiagonalMatrix(Constants.C2, -1, -1);
    }

    @Override
    public Tensor3D getDerivative (Matter matter) {
        return new ZeroTensor(3, 3, 3);
    }
}

package org.proj.physics.coordinate;

import org.proj.math.matrix.Matrix;
import org.proj.math.matrix.special.DiagonalMatrix;
import org.proj.math.vector.Vector;

public class Cartesian implements CoordinateSystem {
    protected Cartesian () {};

    @Override
    public Vector fromCartesianPosition(Vector cartesian) {
        return cartesian;
    }

    @Override
    public Vector toCartesianPosition(Vector position) {
        return position;
    }

    @Override
    public Vector fromCartesianVelocity(Vector position, Vector cartesian) {
        return cartesian;
    }

    @Override
    public Vector toCartesianVelocity(Vector position, Vector velocity) {
        return velocity;
    }
}

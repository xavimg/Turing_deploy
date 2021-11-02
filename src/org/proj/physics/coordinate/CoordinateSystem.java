package org.proj.physics.coordinate;

import org.proj.math.matrix.Matrix;
import org.proj.math.tensor.Tensor3D;
import org.proj.math.vector.Vector;

public interface CoordinateSystem {
    CoordinateSystem CARTESIAN = new Cartesian();
    CoordinateSystem POLAR = new Polar();

    Vector fromCartesianPosition (Vector cartesian);
    Vector toCartesianPosition (Vector position);

    /**
     * @param position Position in cartesian coordinates
     * @param velocity Velocity in cartesian coordinates
     * @return Velocity in original coordinates
     */
    Vector fromCartesianVelocity (Vector position, Vector velocity);

    /**
     * @param position Position in original coordinates
     * @param velocity Velocity in original coordinates
     * @return Velocity in cartesian coordinates
     */
    Vector toCartesianVelocity (Vector position, Vector velocity);
}

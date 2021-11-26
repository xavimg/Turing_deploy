package org.proj.physics.coordinate;

import org.proj.math.vector.Vec2;

public interface CoordinateSystem {
    CoordinateSystem CARTESIAN = new Cartesian();
    CoordinateSystem POLAR = new Polar();

    Vec2 fromCartesian (Vec2 cartesian);
    Vec2 toCartesian (Vec2 position);
}

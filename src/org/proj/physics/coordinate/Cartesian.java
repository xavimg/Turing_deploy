package org.proj.physics.coordinate;


import org.proj.math.vector.Vec2;

public class Cartesian implements CoordinateSystem {
    protected Cartesian () {};

    @Override
    public Vec2 fromCartesian(Vec2 cartesian) {
        return cartesian;
    }

    @Override
    public Vec2 toCartesian(Vec2 position) {
        return position;
    }
}

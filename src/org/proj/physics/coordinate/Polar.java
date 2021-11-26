package org.proj.physics.coordinate;

import org.proj.math.vector.Vec2;

public class Polar implements CoordinateSystem {
    protected Polar () {};

    @Override
    public Vec2 fromCartesian (Vec2 cartesian) {
        double x = cartesian.x;
        double y = cartesian.y;

        return new Vec2(Math.hypot(x, y), Math.atan2(y, x));
    }

    @Override
    public Vec2 toCartesian (Vec2 position) {
        double r = position.x;
        double theta = position.y;

        return new Vec2(r * Math.cos(theta), r * Math.sin(theta));
    }
}

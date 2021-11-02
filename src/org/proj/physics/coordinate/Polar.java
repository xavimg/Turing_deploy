package org.proj.physics.coordinate;

import org.proj.math.matrix.Matrix;
import org.proj.math.vector.Vector;

public class Polar implements CoordinateSystem {
    protected Polar () {};

    @Override
    public Vector fromCartesianPosition (Vector cartesian) {
        double x = cartesian.get(0);
        double y = cartesian.get(1);

        return Vector.of(Math.hypot(x, y), Math.atan2(y, x));
    }

    @Override
    public Vector toCartesianPosition (Vector position) {
        double r = position.get(0);
        double theta = position.get(1);

        return Vector.of(r * Math.cos(theta), r * Math.sin(theta));
    }

    @Override
    public Vector fromCartesianVelocity (Vector position, Vector velocity) {
        double x = position.get(0);
        double y = position.get(1);

        double vx = velocity.get(0);
        double vy = velocity.get(1);

        double r2 = x * x + y * y;
        double r = Math.hypot(x, y);

        return Vector.of((x * vx + y * vy) / r, (x * vy - y * vx) / r2); // CHANGED "/R2" FOR "/R"
    }

    @Override
    public Vector toCartesianVelocity (Vector position, Vector velocity) {
        double r = position.get(0);
        double theta = position.get(1);

        double sin = Math.sin(theta);
        double cos = Math.cos(theta);

        double vr = velocity.get(0);
        double vtheta = velocity.get(1);

        double lambda = r * vtheta;
        return Vector.of((vr * cos) - (lambda * sin), (vr * sin) + (lambda * cos));
    }
}

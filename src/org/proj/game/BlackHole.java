package org.proj.game;

import org.proj.math.vector.Vector;
import org.proj.physics.metric.Schwarzschild;
import org.proj.physics.metric.cartesian.SchwarzschildCartesian;

import java.awt.*;

public class BlackHole extends SpaceBody {
    private BlackHole (double restMass, double radius, double angularVelocity, Vector position, Vector velocity) {
        super(restMass, radius, angularVelocity, position, velocity, new SchwarzschildCartesian(restMass), Color.BLACK, null);
    }

    public static BlackHole ofMass (double mass, double angularVelocity, Vector position, Vector velocity) {
        return new BlackHole(mass, Schwarzschild.radius(mass), angularVelocity, position, velocity);
    }

    public static BlackHole ofRadius (double radius, double angularVelocity, Vector position, Vector velocity) {
        return new BlackHole(Schwarzschild.mass(radius), radius, angularVelocity, position, velocity);
    }
}

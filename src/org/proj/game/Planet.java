package org.proj.game;

import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.metric.Kerr;
import org.proj.physics.metric.MetricTensor;
import org.proj.physics.metric.Schwarzschild;
import org.proj.physics.metric.cartesian.SchwarzschildCartesian;

import java.awt.*;
import java.math.BigDecimal;

/**
 * WARNING: WHEN SENDING POSITION AND VELOCITY, IT MUST BE WITH RESPECT TO THE PLAYER REQUESTING THE INFO
 */
public class Planet extends SpaceBody {
    public Planet (double restMass, double radius, double angularVelocity, Vector position, Vector velocity, Color color, Image texture) {
        super(restMass, radius, angularVelocity, position, velocity, new SchwarzschildCartesian(restMass), color, texture);
    }
}

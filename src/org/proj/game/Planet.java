package org.proj.game;

import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.metric.Kerr;
import org.proj.physics.metric.MetricTensor;
import org.proj.physics.metric.Schwarzschild;

import java.awt.*;
import java.math.BigDecimal;

/**
 * WARNING: WHEN SENDING POSITION AND VELOCITY, IT MUST BE WITH RESPECT TO THE PLAYER REQUESTING THE INFO
 */
public class Planet extends Matter.Defined {
    private Color color; // TODO CALCULATE
    private Image texture; // TODO CALCULATE
    private double surfaceAcceleration;

    private Kerr metric;

    public Planet (double restMass, double radius, double angularVelocity, Vector position, Vector velocity, Color color, Image texture) {
        super(restMass, radius, angularVelocity, position, velocity);
        this.color = color;
        this.texture = texture;

        // CALCULATE SURFACE ACCELERATION USING NEWTONIAN GRAVITY
        this.surfaceAcceleration = Constants.G * restMass / (radius * radius);

        // METRIC
        this.metric = new Kerr(restMass, radius, angularVelocity);
    }
}

package org.proj.game;

import org.proj.math.vector.Vector;
import org.proj.physics.Matter;
import org.proj.physics.metric.MetricTensor;

import java.awt.*;

public class SpaceBody extends Matter.Defined {
    private Color color; // TODO
    private Image texture; // TODO

    public SpaceBody (double restMass, double radius, double angularVelocity, Vector position, Vector velocity, Color color, Image texture) {
        super(restMass, radius, angularVelocity, position, velocity);
        this.color = color;
        this.texture = texture;
    }
}

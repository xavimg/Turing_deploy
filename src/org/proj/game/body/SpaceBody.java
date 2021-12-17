package org.proj.game.body;

import org.proj.math.vector.Vec2;
import org.proj.physics.Matter;
import org.proj.physics.metric.MetricTensor;

import java.awt.*;

public class SpaceBody extends Matter.Defined {
    final public Color color;
    final public Image texture;
    final public MetricTensor metric;

    public SpaceBody (double restMass, double radius, Vec2 position, Vec2 velocity, MetricTensor metric, Color color, Image texture) {
        super(restMass, radius, position, velocity);
        this.color = color;
        this.texture = texture;
        this.metric = metric;
    }
}

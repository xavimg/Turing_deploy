package org.proj.game.body;

import org.proj.math.vector.Vec2;
import org.proj.physics.metric.SchwarzschildCartesian;

import java.awt.*;

/**
 * WARNING: WHEN SENDING POSITION AND VELOCITY, IT MUST BE WITH RESPECT TO THE PLAYER REQUESTING THE INFO
 */
public class Planet extends SpaceBody {
    public Planet (double restMass, double radius, Vec2 position, Vec2 velocity, Color color, Image texture) {
        super(restMass, radius, position, velocity, new SchwarzschildCartesian(restMass), color, texture);
    }
}

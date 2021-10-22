package org.proj.game;

import org.proj.math.vector.Vector;
import org.proj.physics.Matter;

import java.awt.*;
import java.math.BigDecimal;

/**
 * WARNING: WHEN SENDING POSITION AND VELOCITY, IT MUST BE WITH RESPECT TO THE PLAYER REQUESTING THE INFO
 */
public abstract class Planet extends Matter.Defined {
    private Color color;
    private Image texture;

    public Planet (BigDecimal restMass, BigDecimal radius, BigDecimal angularVelocity, Vector position, Vector velocity) {
        super(restMass, radius, angularVelocity, position, velocity);
    }
}

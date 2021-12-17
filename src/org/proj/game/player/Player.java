package org.proj.game.player;

import org.proj.game.PlanetarySystem;
import org.proj.math.vector.Vec2;
import org.proj.utils.Lazy;

/**
 * WARNING: WHEN SENDING POSITION AND VELOCITY, IT MUST BE WITH RESPECT TO PLAYER'S PROPER TIME
 */
public class Player {
    private Lazy<PlanetarySystem> system;
    private Vec2 position, velocity;
}

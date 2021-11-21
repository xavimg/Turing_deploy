package org.proj.physics;

import org.proj.math.vector.Vec2;

public abstract class Matter {
    private Matter () {};

    /**
     * @return Rest mass in solar masses
     */
    public abstract double restMass ();

    /**
     * @return Radius in light seconds
     */
    public abstract double radius ();

    /**
     * @return Spacial position in light seconds relative to origin
     */
    public abstract Vec2 getPosition ();

    /**
     * @return Spacial velocity in light seconds per second relative to origin
     */
    public abstract Vec2 getVelocity ();

    public Matter delta (Vec2 pos, Vec2 vel) {
        return new Matter() {
            @Override
            public double restMass() {
                return Matter.this.restMass();
            }

            @Override
            public double radius() {
                return Matter.this.radius();
            }

            @Override
            public Vec2 getPosition() {
                return pos;
            }

            @Override
            public Vec2 getVelocity() {
                return vel;
            }
        };
    }

    // SUBCLASSES
    public static class Defined extends Matter {
        final private double restMass, radius;
        private Vec2 position, velocity;

        public Defined (double restMass, double radius, Vec2 position, Vec2 velocity) {
            this.restMass = restMass;
            this.radius = radius;
            this.position = position;
            this.velocity = velocity;
        }

        @Override
        public double restMass() {
            return restMass;
        }

        @Override
        public double radius() {
            return radius;
        }

        @Override
        public Vec2 getPosition() {
            return position;
        }

        @Override
        public Vec2 getVelocity() {
            return velocity;
        }

        final public void addVelocity (Vec2 vel) {
            this.velocity = this.velocity.add(vel);
        }

        final public void update (double dt) {
            this.position = this.position.add(velocity.mul(dt));
        }
    }
}

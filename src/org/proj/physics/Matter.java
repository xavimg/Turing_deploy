package org.proj.physics;

import org.proj.math.vector.LazyVector;
import org.proj.math.vector.Vector;
import org.proj.physics.metric.Schwarzschild;

import java.math.BigDecimal;

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
     * @return Angular velocity in radians per second
     */
    public abstract double angularVelocity ();

    /**
     * @return Spacial position in light seconds relative to origin
     */
    public abstract Vector getPosition ();

    /**
     * @return Spacial velocity in light seconds per second relative to origin
     */
    public abstract Vector getVelocity ();

    /**
     * @return Moment of inertia in M☉ * ls<sup>2</sup>
     */
    public double inertia () {
        return radius() * radius() * restMass();
    }

    /**
     * @return Angular momentum in M☉ * ls<sup>2</sup> * s<sup>-1</sup>
     */
    public double angularMomentum () {
        return inertia() * angularVelocity();
    }

    public Matter delta (Vector pos, Vector vel) {
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
            public double angularVelocity() {
                return Matter.this.angularVelocity();
            }

            @Override
            public Vector getPosition() {
                return pos;
            }

            @Override
            public Vector getVelocity() {
                return vel;
            }
        };
    }

    // SUBCLASSES
    public static class Defined extends Matter {
        final private double restMass, radius, angularVelocity;
        final private double inertia, angularMomentum;
        private Vector position, velocity;

        public Defined (double restMass, double radius, double angularVelocity, Vector position, Vector velocity) {
            this.restMass = restMass;
            this.radius = radius;
            this.angularVelocity = angularVelocity;
            this.position = position;
            this.velocity = velocity;

            this.inertia = this.radius * this.radius * this.restMass;
            this.angularMomentum = this.inertia * this.angularVelocity;
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
        public double angularVelocity() {
            return angularVelocity;
        }

        @Override
        public double inertia() {
            return inertia;
        }

        @Override
        public double angularMomentum() {
            return angularMomentum;
        }

        @Override
        public Vector getPosition() {
            return position;
        }

        @Override
        public Vector getVelocity() {
            return velocity;
        }

        final public void addVelocity (Vector vel) {
            this.velocity = this.velocity.add(vel);
        }

        final public void update (double dt) {
            this.position = this.position.add(velocity.mul(dt));
            if (position instanceof LazyVector) {
                this.position = ((LazyVector) this.position).toStatic();
            }
        }
    }
}

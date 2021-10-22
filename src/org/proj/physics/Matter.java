package org.proj.physics;

import org.proj.math.vector.Vector;
import org.proj.physics.metric.Schwarzschild;

import java.math.BigDecimal;

public abstract class Matter {
    /**
     * @return Rest mass in solar masses
     */
    public abstract BigDecimal restMass ();

    /**
     * @return Radius in light seconds
     */
    public abstract BigDecimal radius ();

    /**
     * @return Angular velocity in radians per second
     */
    public abstract BigDecimal angularVelocity ();

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
    public BigDecimal inertia () {
        return radius().pow(2).multiply(restMass());
    }

    /**
     * @return Angular momentum in M☉ * ls<sup>2</sup> * s<sup>-1</sup>
     */
    public BigDecimal angularMomentum () {
        return inertia().multiply(angularVelocity());
    }

    /**
     * Calculates weather matter is a black hole
     * @see Schwarzschild#radius(double)
     * @return {@link Boolean#TRUE} if {@link #radius()} is lower or equal to {@link Schwarzschild#radius(double)}, otherwise {@link Boolean#FALSE}
     */
    final public boolean isBlackHole () {
        return radius().compareTo(Schwarzschild.radius(restMass())) <= 0;
    }

    public Matter delta (Vector pos, Vector vel) {
        return new Matter() {
            @Override
            public BigDecimal restMass() {
                return Matter.this.restMass();
            }

            @Override
            public BigDecimal radius() {
                return Matter.this.radius();
            }

            @Override
            public BigDecimal angularVelocity() {
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
        final private BigDecimal restMass, radius, angularVelocity;
        final private BigDecimal inertia, angularMomentum;

        private Vector position, velocity;

        public Defined (BigDecimal restMass, BigDecimal radius, BigDecimal angularVelocity, Vector position, Vector velocity) {
            this.restMass = restMass;
            this.radius = radius;
            this.angularVelocity = angularVelocity;
            this.position = position;
            this.velocity = velocity;

            this.inertia = this.radius.pow(2).multiply(this.restMass);
            this.angularMomentum = this.inertia.pow(2);
        }

        @Override
        public BigDecimal restMass() {
            return restMass;
        }

        @Override
        public BigDecimal radius() {
            return radius;
        }

        @Override
        public BigDecimal angularVelocity() {
            return angularVelocity;
        }

        @Override
        public BigDecimal inertia() {
            return inertia;
        }

        @Override
        public BigDecimal angularMomentum() {
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
        }
    }
}

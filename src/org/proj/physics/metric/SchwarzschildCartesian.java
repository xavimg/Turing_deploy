package org.proj.physics.metric;

import org.proj.math.Mat3;
import org.proj.math.MathUtils;
import org.proj.math.Tens3;
import org.proj.math.vector.Vec3;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.coordinate.CoordinateSystem;
import java.math.BigDecimal;
import java.math.MathContext;

public class SchwarzschildCartesian extends MetricTensor {
    final public double mass;
    final private double rs;
    final private double isco;

    public SchwarzschildCartesian (double mass) {
        this.mass = mass;
        this.rs = Schwarzschild.radius(mass);
        this.isco = 6 * Constants.G * mass / Constants.C2;
    }

    @Override
    final public CoordinateSystem getCoordinateSystem() {
        return CoordinateSystem.CARTESIAN;
    }

    @Override
    public double getIsco(Matter matter) {
        return isco;
    }

    /**
     * @see <a href="https://math.stackexchange.com/a/3719432">Metric tensor conversion</a>
     */
    @Override
    public Mat3 getMetric (Matter matter) {
        double x = matter.getPosition().x;
        double y = matter.getPosition().y;

        double x2 = x * x;
        double y2 = y * y;

        double r = Math.hypot(x, y);
        double r2 = x2 + y2;

        double alpha = r2 - r * rs;
        double dxdy = (2 * x * y) / (-alpha * r2);

        return new Mat3(
                new Vec3(Constants.C2 * (1 - rs / r), 0, 0),
                new Vec3(0, -(x2 / alpha + y2 / r2), dxdy),
                new Vec3(0, dxdy, -(y2 / alpha + x2 / r2))
        );
    }

    // TODO MANUAL DERIVATIVE, THIS IS SUPER-INEFFICIENT

    @Override
    public Tens3 getDerivative (Matter matter) {
        final BigDecimal two = BigDecimal.valueOf(2);
        final BigDecimal rs = BigDecimal.valueOf(this.rs);
        final BigDecimal c2 = BigDecimal.valueOf(Constants.C2);

        DerivateMetric func = (x,y,i,j) -> {
            BigDecimal x2 = x.pow(2);
            BigDecimal y2 = y.pow(2);
            BigDecimal r2 = x2.add(y2);
            BigDecimal r = r2.sqrt(MathContext.DECIMAL128);
            BigDecimal alpha = r2.subtract(r.multiply(rs));

            final BigDecimal dxdy = two.multiply(x).multiply(y).divide(alpha.negate().multiply(r2), MathContext.DECIMAL128);
            return switch (i) {
                case 0 -> j == 0 ? c2.multiply(BigDecimal.ONE.subtract(rs.divide(r, MathContext.DECIMAL128))) : BigDecimal.ZERO;

                case 1 -> switch (j) {
                    case 1 -> x2.negate().divide(alpha, MathContext.DECIMAL128).subtract(y2.divide(r2, MathContext.DECIMAL128));
                    case 2 -> dxdy;
                    default -> BigDecimal.ZERO;
                };

                case 2 -> switch (j) {
                    case 1 -> dxdy;
                    case 2 -> y2.negate().divide(alpha, MathContext.DECIMAL128).subtract(x2.divide(r2, MathContext.DECIMAL128));
                    default -> BigDecimal.ZERO;
                };

                default -> BigDecimal.ZERO;
            };
        };

        BigDecimal X = BigDecimal.valueOf(matter.getPosition().x);
        BigDecimal Y = BigDecimal.valueOf(matter.getPosition().y);

        return Tens3.of((i,j,k) -> {
            return switch (i) {
                case 1 -> MathUtils.derivative(X, (BigDecimal x) -> func.apply(x, Y, j, k));
                case 2 -> MathUtils.derivative(Y, (BigDecimal y) -> func.apply(X, y, j, k));
                default -> 0;
            };
        });
    }

    interface DerivateMetric {
        BigDecimal apply (BigDecimal x, BigDecimal y, int i, int j);
    }
}

package org.proj.game;

import org.proj.math.MathUtils;
import org.proj.math.matrix.Matrix;
import org.proj.math.vector.Vector;
import org.proj.physics.Constants;
import org.proj.physics.Matter;
import org.proj.physics.metric.Kerr;
import org.proj.physics.metric.MetricTensor;
import org.proj.physics.metric.Schwarzschild;

import java.awt.*;

public class Sun extends SpaceBody {
    final private static double nanoC = 2.998e17;
    final private static double nanoC2 = nanoC * nanoC;

    final private static Matrix rgbMatrix = Matrix.of(
            Vector.of(0.67, 0.33, 0),
            Vector.of(0.21, 0.71, 0.08),
            Vector.of(0.15, 0.06, 0.79)
    ).transp().inverse();
    final private static Vector whiteScale = Vector.of(0.3101, 0.3162, 0.3737);

    final public double temperature; // in kelvin

    public Sun (double restMass, double radius, double angularVelocity, double temperature) {
        super(restMass, radius, angularVelocity, null, null, new Schwarzschild(restMass), getColor(temperature), null);
        this.temperature = temperature;
    }

    public Sun (double temperature, double angularVelocity) {
        this(0.513829 * Math.exp(0.000114646 * temperature), 0.725841 * Math.exp(0.000073683 * temperature) * 2.32061, angularVelocity, temperature);
    }

    public Sun (double temperature, double radius, double angularVelocity) {
        this(0.513829 * Math.exp(0.000114646 * temperature), radius, angularVelocity, temperature);
    }

    // PRIVATE STATIC
    /**
     * @see <a href="http://www.fourmilab.ch/documents/specrend/">Usefull source</a>
     */
    private static Color getColor (double temperature) {
        double X = MathUtils.integral(380, 780, Integer.MAX_VALUE, x -> spectralRadiance(x, temperature) * xFunction(x));
        double Y = MathUtils.integral(380, 780, Integer.MAX_VALUE, x -> spectralRadiance(x, temperature) * yFunction(x));
        double Z = MathUtils.integral(380, 780, Integer.MAX_VALUE, x -> spectralRadiance(x, temperature) * zFunction(x));

        Vector xyz = Vector.of(X, Y, Z);
        xyz = xyz.div(xyz.sum());

        Vector rgb = rgbMatrix.mul(xyz).div(whiteScale);
        return new Color(MathUtils.clamp((float) rgb.get(0), 0, 1), MathUtils.clamp((float) rgb.get(1), 0, 1), MathUtils.clamp((float) rgb.get(2), 0, 1));
    }

    private static double spectralRadiance (double lambda, double T) {
        return 2 * Constants.H * nanoC2 / (Math.pow(lambda, 5) * (Math.exp(Constants.H * nanoC / (lambda * Constants.K * T)) - 1));
    }

    private static double gaussianFunc (double x, double mu, double gamma1, double gamma2) {
        return Math.exp(-Math.pow(x - mu, 2) / (2 * Math.pow(x < mu ? gamma1 : gamma2, 2)));
    }

    private static double xFunction (double lambda) {
        return 1.056 * gaussianFunc(lambda, 599.8, 37.9, 31)
                + 0.362 * gaussianFunc(lambda, 442, 16, 26.7)
                - 0.065 * gaussianFunc(lambda, 501.1, 20.4, 26.2);
    }

    private static double yFunction (double lambda) {
        return 0.821 * gaussianFunc(lambda, 568.8, 46.9, 40.5)
                + 0.286 * gaussianFunc(lambda, 530.9, 16.3, 31.1);
    }

    private static double zFunction (double lambda) {
        return 1.217 * gaussianFunc(lambda, 437, 11.8, 36)
                + 0.681 * gaussianFunc(lambda, 459, 26, 13.8);
    }
}

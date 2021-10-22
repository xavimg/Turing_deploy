package org.proj.physics;

import org.proj.math.MathUtils;

import java.math.BigDecimal;

/**
 * Constants used for the calculations
 * <ul>
 *     <li><b>Time</b>: Seconds (s)</li>
 *     <li><b>Distance</b>: Light seconds (ls)</li>
 *     <li><b>Mass</b>: Solar masses (M☉)</li>
 *     <li><b>Velocity</b>: Light seconds per second (ls/s)</li>
 *     <li><b>Angular velocity</b>: Radians per second (rad/s)</li>
 * </ul>
 */
public class Constants {
    /**
     * Speed of light in light seconds per second
     */
    final public static BigDecimal C = BigDecimal.ONE;

    /**
     * Gravitational constant in ls<sup>3</sup> * M☉<sup>-1</sup> * s<sup>-2</sup>
     * @see <a href="https://www.wolframalpha.com/input/?i=gravitational+constant+in+light+seconds+cubed+per+M⊙+per+second+squared">WolframAlpha</a>
     */
    final public static BigDecimal G = BigDecimal.valueOf(4.93e-6);

    final public static double C2 = C * C;
    final public static double C4 = C2 * C2;
    final public static double K = 8 * Math.PI * G / C2;

    /**
     * @param m Distance in meters
     * @return Distance in light seconds
     */
    public static double fromMeters (double m) {
        return m / 299792458d;
    }

    /**
     * @param km Distance in kilometers
     * @return Distance in light seconds
     */
    public static double fromKiloMeters (double km) {
        return km / 299792.458d;
    }

    /**
     * @param kg Mass in kilograms
     * @return Mass in solar masses
     */
    public static double fromKiloGrams (double kg) {
        return kg / 1.998e30d;
    }

    /**
     * @param revs Angular velocity in revolutions per second
     * @param r Radius of rotation
     * @return Angular velocity in radians per second
     */
    public static double fromRevolutionsSecond (double revs, double r) {
        return MathUtils.PI_2 * r * revs;
    }
}

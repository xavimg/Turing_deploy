package org.proj.physics;

import org.proj.math.MathUtils;

import java.math.BigDecimal;
import java.math.MathContext;

/**
 * Constants used for the calculations
 * <ul>
 *     <li><b>Time</b>: Seconds (s)</li>
 *     <li><b>Distance</b>: Light seconds (ls)</li>
 *     <li><b>Mass</b>: Solar masses (M☉)</li>
 *     <li><b>Velocity</b>: Light seconds per second (ls/s)</li>
 *     <li><b>Angular velocity</b>: Radians per second (rad/s)</li>
 *     <li><b>Temperature</b>: Kelvin (K)</li>
 * </ul>
 */
public class Constants {
    /**
     * Speed of light in light seconds per second
     */
    final public static double C = 1;

    /**
     * Gravitational constant in ls<sup>3</sup> * M☉<sup>-1</sup> * s<sup>-2</sup>
     * @see <a href="https://www.wolframalpha.com/input/?i=gravitational+constant+in+light+seconds+cubed+per+M⊙+per+second+squared">WolframAlpha</a>
     */
    final public static double G = 4.93e-6;

    /**
     * Planck's constant
     */
    final public static double H = 6.626e-34;

    /**
     * Boltzmann's constant
     */
    final public static double K = 1.381e-23;

    /**
     * Avogadro's constant
     */
    final public static double NA = 6.02214076e23;

    final public static double C2 = C * C;
    final public static double C4 = C2 * C2;
}

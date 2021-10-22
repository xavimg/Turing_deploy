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

    final public static BigDecimal C2 = C.pow(2);
    final public static BigDecimal C4 = C2.pow(2);
    final public static BigDecimal K = MathUtils.PI.multiply(G).multiply(BigDecimal.valueOf(8)).divide(C2, MathContext.DECIMAL128);
}

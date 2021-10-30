package org.proj.utils;

import org.proj.math.matrix.Matrix;
import org.proj.math.tensor.Tensor3D;
import org.proj.math.vector.Vector;

import java.util.Random;
import java.util.concurrent.atomic.AtomicLong;
import java.util.random.RandomGenerator;

/**
 * Perlin noise adaptation to Java
 * @author Ken Perlin
 * @author Alex Andreba
 * @see <a href="https://mrl.cs.nyu.edu/~perlin/doc/oscar.html#noise">Original source code</a>
 */
public class PerlinNoise {
    final private static int B = 0x100;
    final private static int BM = 0xff;

    final private static int N = 0x1000;
    final private static int NP = 12;
    final private static int NM = 0xfff;

    final private int[] p;
    final private Vector g1;
    final private Matrix g2, g3;

    public PerlinNoise (Random random) {
        // INITIALIZE VARIABLES
        int lambda = B + B + 2;
        this.p = new int[lambda];
        double[] g1 = new double[lambda];
        double[][] g2 = new double[lambda][2];
        double[][] g3 = new double[lambda][3];

        // INITIALIZE VALUES
        int i, j, k;

        for (i=0;i<B;i++) {
            p[i] = i;
            g1[i] = random(random);
            g2[i] = Vector.of(random(random), random(random)).unit().toArray();
            g3[i] = Vector.of(random(random), random(random), random(random)).unit().toArray();
        }

        while (--i >= 0) {
            k = p[i];
            p[i] = p[j = (int) (random.nextLong(0, 2147483647) % B)];
            p[j] = k;
        }

        for (i=0;i<B+2;i++) {
            lambda = B + i;

            p[lambda] = p[i];
            g1[lambda] = g1[i];

            g2[lambda][0] = g2[i][0];
            g2[lambda][1] = g2[i][1];

            g3[lambda][0] = g3[i][0];
            g3[lambda][1] = g3[i][1];
            g3[lambda][2] = g3[i][2];
        }

        // SET VARIABLES
        this.g1 = Vector.of(g1);
        this.g2 = Matrix.of(g2);
        this.g3 = Matrix.of(g3);
    }

    public PerlinNoise (long seed) {
        this(new Random(seed));
    }

    public PerlinNoise () {
        this(new Random());
    }

    public double noise1 (double arg) {
        Vector vec = Vector.of(arg);
        var setup = this.setup(vec, 0);

        int[] bx = setup.first;
        Vector rx = setup.last;

        double sx = sCurve(setup.last.get(0));
        double u = rx.get(0) * g1.get(p[bx[0]]);
        double v = rx.get(1) * g1.get(p[bx[1]]);

        return lerp(sx, u, v);
    }

    public double noise2 (Vector vec) {
        var setup0 = setup(vec, 0);
        var setup1 = setup(vec, 1);

        int[] bx = setup0.first;
        int[] by = setup1.first;

        Vector rx = setup0.last;
        Vector ry = setup1.last;

        int i = p[bx[0]];
        int j = p[bx[1]];

        int[][] b = new int[][] {
                { p[i + by[0]], p[i + by[1]] },
                { p[j + by[0]], p[j + by[1]] }
        };

        double sx = sCurve(rx.get(0));
        double sy = sCurve(ry.get(0));

        Matrix UV = new Matrix (2, 2) {
            @Override
            public double get (int i, int j) {
                return at2(rx.get(i), ry.get(j), g2.get(b[i][j]));
            }
        };

        double A = lerp(sx, UV.get(0, 0), UV.get(1, 0));
        double B = lerp(sx, UV.get(0, 1), UV.get(1, 1));

        return lerp(sy, A, B);
    }

    public double noise2 (double x, double y) {
        return noise2(Vector.of(x, y));
    }

    // PRIVATE
    private static double sCurve (double t) {
        return t * t * (3 - 2 * t);
    }

    private static double lerp (double t, double a, double b) {
        return a + t * (b - a);
    }

    private static double at2 (double rx, double ry, Vector q) {
        return rx * q.get(0) + ry * q.get(1);
    }

    private Couple<int[], Vector> setup (Vector vec, int i) {
        double t = vec.get(i) + N;
        int intT = (int) t;

        int b0 = intT & BM;
        int b1 = (b0 + 1) & BM;

        double r0 = t - intT;
        double r1 = r0 - 1;

        return new Couple<>(new int[]{ b0, b1 }, Vector.of(r0, r1));
    }

    private static double random (Random random) {
        return (double) ((random.nextLong(0, 2147483647) % (B + B)) - B) / B;
    }
}

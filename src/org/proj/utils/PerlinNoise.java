package org.proj.utils;

import org.proj.math.matrix.Mat2;
import org.proj.math.vector.Vec2;
import org.proj.math.vector.Vec3;

import java.util.Random;

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
    final private double[] g1;
    final private Vec2[] g2;
    final private Vec3[] g3;

    public PerlinNoise (Random random) {
        // INITIALIZE VARIABLES
        int lambda = B + B + 2;
        this.p = new int[lambda];
        this.g1 = new double[lambda];
        this.g2 = new Vec2[lambda];
        this.g3 = new Vec3[lambda];

        // INITIALIZE VALUES
        int i, j, k;

        for (i=0;i<B;i++) {
            p[i] = i;
            g1[i] = random(random);
            g2[i] = new Vec2(random(random), random(random)).unit();
            g3[i] = new Vec3(random(random), random(random), random(random)).unit();
        }

        while (--i >= 0) {
            k = p[i];
            p[i] = p[j = (int) (Integer.toUnsignedLong(random.nextInt()) % B)];
            p[j] = k;
        }

        for (i=0;i<B+2;i++) {
            lambda = B + i;

            p[lambda] = p[i];
            g1[lambda] = g1[i];
            g2[lambda] = g2[i];
            g3[lambda] = g3[i];
        }
    }

    public PerlinNoise (long seed) {
        this(new Random(seed));
    }

    public PerlinNoise () {
        this(new Random());
    }

    public double noise1 (double arg) {
        var setup = this.setup(arg);

        int[] bx = setup.first;
        Vec2 rx = setup.last;

        double sx = sCurve(setup.last.get(0));
        double u = rx.x * g1[p[bx[0]]];
        double v = rx.y * g1[p[bx[1]]];

        return lerp(sx, u, v);
    }

    public double noise2 (Vec2 vec) {
        var setup0 = setup(vec.x);
        var setup1 = setup(vec.y);

        int[] bx = setup0.first;
        int[] by = setup1.first;

        Vec2 rx = setup0.last;
        Vec2 ry = setup1.last;

        int i = p[bx[0]];
        int j = p[bx[1]];

        int[][] b = new int[][] {
                { p[i + by[0]], p[i + by[1]] },
                { p[j + by[0]], p[j + by[1]] }
        };

        double sx = sCurve(rx.x);
        double sy = sCurve(ry.x);

        Mat2 UV = Mat2.of((x, y) -> at2(rx.get(x), ry.get(y), g2[b[x][y]]));
        double A = lerp(sx, UV.x.x, UV.y.x);
        double B = lerp(sx, UV.x.y, UV.y.y);

        return lerp(sy, A, B);
    }

    public double noise2 (double x, double y) {
        return noise2(new Vec2(x, y));
    }

    // PRIVATE
    private static double sCurve (double t) {
        return t * t * (3 - 2 * t);
    }

    private static double lerp (double t, double a, double b) {
        return a + t * (b - a);
    }

    private static double at2 (double rx, double ry, Vec2 q) {
        return rx * q.get(0) + ry * q.get(1);
    }

    private Couple<int[], Vec2> setup (double t) {
        t += N;
        int intT = (int) t;

        int b0 = intT & BM;
        int b1 = (b0 + 1) & BM;

        double r0 = t - intT;
        double r1 = r0 - 1;

        return new Couple<>(new int[]{ b0, b1 }, new Vec2(r0, r1));
    }

    private static double random (Random random) {
        return (double) (((long) random.nextInt() % (B + B)) - B) / B;
    }
}

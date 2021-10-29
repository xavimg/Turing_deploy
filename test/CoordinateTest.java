import org.junit.Assert;
import org.junit.Test;
import org.proj.math.MathUtils;
import org.proj.math.Rand;
import org.proj.math.vector.Vector;
import org.proj.physics.coordinate.CoordinateSystem;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

public class CoordinateTest {
    @Test
    public void testPos () {
        class TestEntry {
            final public Vector polar, cart, back;

            public TestEntry(Vector polar, Vector cart, Vector back) {
                this.polar = polar;
                this.cart = cart;
                this.back = back;
            }

            @Override
            public String toString() {
                return "TestEntry{" +
                        "polar=" + polar +
                        ", cart=" + cart +
                        ", back=" + back +
                        '}';
            }
        }

        ArrayList<TestEntry> deltas = new ArrayList<>();
        for (double r=0.01d;r<=2;r+=0.01d) {
            for (double theta = -Math.PI; theta <= Math.PI ; theta+=0.01d) {
                Vector polar = Vector.of(r, theta);
                Vector cart = CoordinateSystem.POLAR.toCartesianPosition(polar);
                Vector back = CoordinateSystem.POLAR.fromCartesianPosition(cart);

                deltas.add(new TestEntry(polar, cart, back));
            }
        }

        Optional<TestEntry> sorted = deltas.stream().max(Comparator.comparingDouble(x -> x.polar.subtr(x.back).length()));
    }

    @Test
    public void testCartPos() {
        class Entry {
            final public Vector cart, polar, back;

            public Entry (Vector cart, Vector polar, Vector back) {
                this.cart = cart;
                this.polar = polar;
                this.back = back;
            }

            public double error () {
                return cart.subtr(back).length();
            }
        }

        ArrayList<Entry> list = new ArrayList<>();

        for (double x=-5d;x<=5d;x+=0.01d) {
            for (double y=-5d;y<=5d;y+=0.01d) {
                Vector cart = Vector.of(x, y);
                Vector polar = CoordinateSystem.POLAR.fromCartesianPosition(cart);
                Vector back = CoordinateSystem.POLAR.toCartesianPosition(polar);

                list.add(new Entry(cart, polar, back));
            }
        }

        var sorted = list.parallelStream().sorted(Comparator.comparingDouble(Entry::error).reversed()).limit(10).collect(Collectors.toUnmodifiableList());
        System.out.println();
    }

    @Test
    public void testVel() {
        class EntryTest {
            final public Vector polarPos, cartPos, polar, cart, back;

            public EntryTest(Vector polarPos, Vector cartPos, Vector polar, Vector cart, Vector back) {
                this.polarPos = polarPos;
                this.cartPos = cartPos;
                this.polar = polar;
                this.cart = cart;
                this.back = back;
            }

            public double error () {
                return polar.subtr(back).length();
            }
        }

        ArrayList<EntryTest> entries = new ArrayList<>();

        for (double vr=-2d;vr<=2d;vr+=0.01d) {
            for (double vtheta=-5d;vtheta<=5d;vtheta+=0.01d) {
                double r = Rand.nextDouble(0, 5);
                double theta = Rand.nextDouble(-Math.PI, Math.PI);

                Vector pos = Vector.of(r, theta);
                Vector cartPos = CoordinateSystem.POLAR.toCartesianPosition(pos);

                Vector vel = Vector.of(vr, vtheta);
                Vector cart = CoordinateSystem.POLAR.toCartesianVelocity(pos, vel);
                Vector back = CoordinateSystem.POLAR.fromCartesianVelocity(cartPos, cart);

                entries.add(new EntryTest(pos, cartPos, vel, cart, back));
            }
        }

        var sorted = entries.parallelStream().sorted(Comparator.comparingDouble(EntryTest::error).reversed()).limit(10).collect(Collectors.toUnmodifiableList());
        System.out.println();
    }

    @Test
    public void testCartVel() {
        class Entry {
            final public Vector cart, polar, back;

            public Entry (Vector cart, Vector polar, Vector back) {
                this.cart = cart;
                this.polar = polar;
                this.back = back;
            }

            public double error () {
                return cart.subtr(back).length();
            }
        }

        ArrayList<Entry> list = new ArrayList<>();

        for (double vx=-5d;vx<=5d;vx+=0.01d) {
            for (double vy=-5d;vy<=5d;vy+=0.01d) {
                double x = Rand.nextDouble(-5, 5);
                double y = Rand.nextDouble(-5, 5);

                Vector pos = Vector.of(x, y);
                Vector polarPos = CoordinateSystem.POLAR.fromCartesianPosition(pos);

                Vector cart = Vector.of(vx, vy);
                Vector polar = CoordinateSystem.POLAR.fromCartesianVelocity(pos, cart);
                Vector back = CoordinateSystem.POLAR.toCartesianVelocity(polarPos, polar);

                list.add(new Entry(cart, polar, back));
            }
        }

        var sorted = list.parallelStream().sorted(Comparator.comparingDouble(Entry::error).reversed()).limit(10).collect(Collectors.toUnmodifiableList());
        System.out.println();
    }
}

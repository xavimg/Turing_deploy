import org.proj.math.Rand;
import org.proj.math.matrix.LazyMatrix;
import org.proj.math.matrix.Matrix;
import org.proj.math.matrix.special.DiagonalMatrix;
import org.proj.math.vector.LazyVector;

public class MatrixTest {
    public static void main (String... args) {
        LazyMatrix alpha = new LazyMatrix (3, 3) {
            @Override
            public double compute(int i, int j) {
                return Rand.nextDouble(-10, 10);
            }
        };

        DiagonalMatrix beta = new DiagonalMatrix(new LazyVector(3) {
            @Override
            public double compute (int pos) {
                return pos + 1;
            }
        });

        System.out.println(alpha);
        System.out.println(beta);
        System.out.println();
        System.out.println(alpha.inverse());
        System.out.println(beta.inverse());
    }
}

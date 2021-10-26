package org.proj.math.tensor;

import org.proj.math.matrix.Matrix;
import org.proj.math.matrix.special.ZeroMatrix;
import org.proj.math.vector.Vector;
import org.proj.math.vector.special.ZeroVector;

import java.math.BigDecimal;

public class ZeroTensor extends Tensor3D {
    public ZeroTensor(int alpha, int beta, int gamma) {
        super(alpha, beta, gamma);
    }

    @Override
    public double get (int x, int y, int z) {
        return 0;
    }

    @Override
    public ZeroMatrix get (int i) {
        return new ZeroMatrix(beta, gamma);
    }

    @Override
    public ZeroVector get (int i, int j) {
        return new ZeroVector(gamma);
    }
}

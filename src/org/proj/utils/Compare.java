package org.proj.utils;

public class Compare {
    public static <A extends Comparable<B>, B> boolean isEqual (A alpha, B beta) {
        return alpha.compareTo(beta) == 0;
    }

    public static <A extends Comparable<B>, B> boolean isGreater (A alpha, B beta) {
        return alpha.compareTo(beta) > 0;
    }

    public static <A extends Comparable<B>, B> boolean isLesser (A alpha, B beta) {
        return alpha.compareTo(beta) < 0;
    }

    public static <A extends Comparable<B>, B> boolean isGreaterOrEqual (A alpha, B beta) {
        return alpha.compareTo(beta) >= 0;
    }

    public static <A extends Comparable<B>, B> boolean isLesserOrEqual (A alpha, B beta) {
        return alpha.compareTo(beta) <= 0;
    }
}

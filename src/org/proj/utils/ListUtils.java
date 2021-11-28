package org.proj.utils;

import java.util.List;

public class ListUtils {
    public static boolean[] toBoolArray (List<Boolean> list) {
        boolean[] array = new boolean[list.size()];
        Range.ofInt(0, array.length, true).forEach(i -> array[i] = list.get(i));

        return array;
    }

    public static byte[] toByteArray (List<Number> list) {
        byte[] array = new byte[list.size()];
        Range.ofInt(0, array.length, true).forEach(i -> array[i] = list.get(i).byteValue());

        return array;
    }

    public static short[] toShortArray (List<Number> list) {
        short[] array = new short[list.size()];
        Range.ofInt(0, array.length, true).forEach(i -> array[i] = list.get(i).shortValue());

        return array;
    }

    public static char[] toCharArray (List<Character> list) {
        char[] array = new char[list.size()];
        Range.ofInt(0, array.length, true).forEach(i -> array[i] = list.get(i));

        return array;
    }

    public static int[] toIntArray (List<Number> list) {
        int[] array = new int[list.size()];
        Range.ofInt(0, array.length, true).forEach(i -> array[i] = list.get(i).intValue());

        return array;
    }

    public static long[] toLongArray (List<Number> list) {
        long[] array = new long[list.size()];
        Range.ofInt(0, array.length, true).forEach(i -> array[i] = list.get(i).longValue());

        return array;
    }

    public static float[] toFloatArray (List<Number> list) {
        float[] array = new float[list.size()];
        Range.ofInt(0, array.length, true).forEach(i -> array[i] = list.get(i).floatValue());

        return array;
    }

    public static double[] toDoubleArray (List<Number> list) {
        double[] array = new double[list.size()];
        Range.ofInt(0, array.length, true).forEach(i -> array[i] = list.get(i).doubleValue());

        return array;
    }
}

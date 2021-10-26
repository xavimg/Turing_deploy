package org.proj.utils;

import java.util.function.LongConsumer;

public class ThreadUtils {
    public static Thread interval (long interval, LongConsumer run) {
        return new Thread(() -> {
            long lastDelta = 0;

            while (true) {
                long start = System.currentTimeMillis();
                run.accept(lastDelta);
                long end = System.currentTimeMillis();

                long delta = end - start;
                long d = interval - delta;
                if (d > 0) {
                    try {
                        Thread.sleep(d);
                    } catch (Exception e) {
                        e.printStackTrace();
                        break;
                    }
                }

                lastDelta = delta;
            }
        });
    };
}

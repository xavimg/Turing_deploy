package org.proj.utils;

import java.util.function.LongConsumer;

public class ThreadUtils {
    public static Thread interval (long interval, Runnable run) {
        return new Thread(() -> {
            while (true) {
                long start = System.currentTimeMillis();
                run.run();
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
            }
        });
    };
}

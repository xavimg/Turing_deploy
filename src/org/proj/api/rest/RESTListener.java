package org.proj.api.rest;

import com.sun.net.httpserver.HttpServer;
import org.bson.json.JsonWriter;

public class RESTListener extends Thread {
    final private HttpServer server;

    public RESTListener (HttpServer server) {
        this.server = server;
    }

    @Override
    public synchronized void start() {
        // STATUS CONTEXT
        server.createContext("/", exchange -> {
           //exchange.sendResponseHeaders();
        });

        server.start();
    }
}

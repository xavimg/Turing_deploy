package org.proj.api;

import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpServer;
import org.bson.json.JsonWriter;
import org.proj.data.json.JSONObject;

import java.io.IOException;
import java.net.InetSocketAddress;

public class RESTListener {
    final private HttpServer server;
    private long startTime;

    public RESTListener (HttpServer server) {
        this.server = server;
    }

    public RESTListener (int port) throws IOException {
        this(HttpServer.create(new InetSocketAddress(port), 0));
    }

    public void start() {
        this.startTime = System.currentTimeMillis();
        server.createContext("/status", this::status);
        server.start();
    }

    private void status (HttpExchange exchange) throws IOException {
        long time = System.currentTimeMillis();

        JSONObject response = new JSONObject();
        response.put("running", true);
        response.put("local_time", time);
        response.put("uptime", time - this.startTime);

        ApiUtils.sendResponse(exchange, 200, response);
    }

    public static void main (String... args) throws IOException {
        RESTListener server = new RESTListener(9876);
        server.start();
    }
}

package org.proj.api;

import com.sun.net.httpserver.HttpExchange;
import org.proj.data.json.JSONObject;

import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;

public class ApiUtils {
    public static void sendResponse (HttpExchange exchange, int code, byte... response) throws IOException {
        exchange.sendResponseHeaders(code, response.length);
        OutputStream os = exchange.getResponseBody();
        os.write(response);
        os.close();
    }

    public static void sendResponse (HttpExchange exchange, int code, String response) throws IOException {
        sendResponse(exchange, code, response.getBytes(StandardCharsets.UTF_8));
    }

    public static void sendResponse (HttpExchange exchange, int code, JSONObject json) throws IOException {
        exchange.getResponseHeaders().set("Content-Type", "application/json");
        sendResponse(exchange, code, json.toJsonString());
    }
}

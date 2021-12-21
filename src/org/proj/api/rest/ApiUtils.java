package org.proj.api.rest;

import com.sun.net.httpserver.HttpExchange;
import org.json.simple.JSONObject;
import org.rol.ReadOnlyMap;

import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;

public class ApiUtils {
    public static ReadOnlyMap<String, String> getQuery (HttpExchange exchange) {
        HashMap<String, String> map = new HashMap<>();
        String[] query = exchange.getRequestURI().getQuery().split("&");

        for (String element: query) {
            String[] pair = element.split("=");
            map.put(pair[0], pair[1]);
        }

        return ReadOnlyMap.ofMap(map);
    }

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
        sendResponse(exchange, code, json.toJSONString());
    }
}

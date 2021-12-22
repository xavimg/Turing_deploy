package org.proj.api

import com.sun.net.httpserver.HttpHandler
import com.sun.net.httpserver.HttpServer
import org.json.simple.JSONObject
import org.proj.api.rest.ApiUtils
import org.sjr.JSONObj
import java.net.InetSocketAddress
import java.util.*
import kotlin.collections.HashMap

enum class RequestType {
    GET,
    POST
}

class RestManager (var server: HttpServer): Thread() {
    var handlers: HashMap<String, EnumMap<RequestType, HttpHandler>> = HashMap();

    constructor(port: Int) : this(HttpServer.create(InetSocketAddress(port), 0))

    fun context (type: RequestType, path: String, handler: HttpHandler) {
        var current = this.handlers.computeIfAbsent(path) { EnumMap(RequestType::class.java) }
        current[type] = handler
    }

    fun get (path: String, handler: HttpHandler) {
        context(RequestType.GET, path, handler)
    }

    fun post (path: String, handler: HttpHandler) {
        context(RequestType.POST, path, handler)
    }

    override fun run () {
        for (handler in handlers) {
            this.server.createContext(handler.key) { e ->
                for (type in handler.value) {
                    if (type.key.name == e.requestMethod) {
                        type.value.handle(e)
                        return@createContext
                    }
                }

                var error = JSONObj()
                error.put("error", "Invalid request");
                ApiUtils.sendResponse(e, 200, error)
            }
        }

        this.server.start()
    }
}
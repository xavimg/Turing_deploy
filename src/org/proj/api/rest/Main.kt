package org.proj.api

import org.proj.api.rest.ApiUtils
import org.proj.json.JSONObject

fun main (args: Array<String>) {
    var port = args[0].toInt();
    var server = RestManager(port);

    server.get("/status") { e ->
        var response = JSONObject()
        response.put("running", true)
        ApiUtils.sendResponse(e, 200, response)
    }

    server.run()
}
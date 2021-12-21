package org.proj.api.rest

import org.json.simple.JSONObject
import org.proj.api.RestManager
import org.proj.game.resource.PTElement
import org.proj.json.codec.PTElementEncoder

fun main (args: Array<String>) {
    var port = args[0].toInt();
    var server = RestManager(port);

    server.get("/status") { e ->
        var response = JSONObject()
        response.put("running", true)
        ApiUtils.sendResponse(e, 200, response)
    }

    /*server.get("/resource") { e ->
        println("a")
        var response = JSONObject()
        println("b")
        println(PTElement.ELEMENTS[0])
        response.put("elements", PTElementEncoder.INSTANCE.encode(PTElement.ELEMENTS));
        println("c")
        ApiUtils.sendResponse(e, 200, response)
        println("d")
    }*/

    server.run()
}
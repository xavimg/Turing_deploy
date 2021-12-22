package org.proj.api.rest

import org.proj.api.RestManager
import org.proj.game.resource.PTElement
import org.proj.json.PTElementEncoder
import org.rol.ReadOnlyList
import org.sjr.JSONObj
import org.sjr.Result
import java.io.InputStreamReader
import java.lang.Exception
import java.net.URL
import java.util.*

fun main (args: Array<String>) {
    var port = args[0].toInt();
    var server = RestManager(port);

    server.get("/status") { e ->
        var response = JSONObj()
        response.put("running", true)
        ApiUtils.sendResponse(e, 200, response)
    }

    server.get("/resource") { e ->
        var response = JSONObj()
        response.put("elements", PTElementEncoder.INSTANCE, PTElement.ELEMENTS);
        ApiUtils.sendResponse(e, 200, response)
    }

    server.run()
    /*var response = JSONObj()
    response.put("elements", PTElementEncoder.INSTANCE, PTElement.ELEMENTS);
    println(response.toJSONString())*/
}
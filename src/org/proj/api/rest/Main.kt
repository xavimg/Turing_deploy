package org.proj.api.rest

import org.proj.api.RestManager
import org.proj.game.Aliment
import org.proj.game.RawElement
import org.proj.game.Resource
import org.proj.json.resource.JAlimentEncoder
import org.sjr.JSONObj

fun main (args: Array<String>) {
    var port = args[0].toInt();
    var server = RestManager(port);

    server.get("/status") { e ->
        var response = JSONObj()
        response.put("running", true)
        ApiUtils.sendResponse(e, 200, response)
    }

    server.get("/resource") { e ->
        var response = JSONObj();
        response.put("aliments", JAlimentEncoder.INSTANCE, *Aliment.values())
        ApiUtils.sendResponse(e, 200, response)
    }

    println("Server open!!")
    //server.run()

    println(Resource.ALL.sortedBy { x -> x.value } .map { x -> Pair(x.name, x.value) })
}
package org.proj.api.rest

import org.proj.api.RestManager
import org.proj.db.Database
import org.proj.game.Resource
import org.proj.json.resource.JResourceEncoderSupplier
import org.sjr.ExceptionEncoder
import org.sjr.JSONObj
import javax.xml.crypto.Data

fun main (args: Array<String>) {
    var port = args[0].toInt();
    var server = RestManager(port);

    Database.forceInit()

    server.get("/status") { e ->
        var response = JSONObj()
        response.put("running", true)
        ApiUtils.sendResponse(e, 200, response)
    }

    server.get("/db") { e ->
        var response = JSONObj()
        response.put("collection", Database.SYSTEMS.countDocuments())
        ApiUtils.sendResponse(e, 200, response)
    }

    server.get("/resources") { e ->
        var response = JSONObj()
        var test = response.put("resources", JResourceEncoderSupplier, Resource.ALL)

        if (test.isPresent) {
            test.get().printStackTrace()

            var error = JSONObj()
            error.put("error", test.get().message)
            ApiUtils.sendResponse(e, 500, error)
            return@get
        }

        ApiUtils.sendResponse(e, 200, response)
    }

    println("Server open!!")
    server.run()
}
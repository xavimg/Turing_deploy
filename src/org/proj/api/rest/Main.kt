package org.proj.api.rest

import org.proj.api.RestManager
import org.proj.db.Database
import org.proj.game.Resource
import org.proj.json.resource.JResourceEncoderSupplier
import org.sjr.JSONObj
import javax.xml.crypto.Data
import kotlin.system.exitProcess

fun main (args: Array<String>) {
    var port = args[0].toInt();
    var server = RestManager(port);

    server.get("/status") { e ->
        var response = JSONObj()
        response.put("running", true)
        response.put("database", Database.ping())
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
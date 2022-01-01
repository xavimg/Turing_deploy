package org.proj.json.resource

import org.proj.game.Resource
import org.sjr.JSONObj
import org.sjr.codec.JSONEncoder

object JResourceEncoder: JSONEncoder<Resource> {
    override fun encode (entry: Resource): JSONObj {
        var response = JSONObj()
        response.put("name", entry.name)
        response.put("size", entry.size)
        response.put("probability", entry.probability)
        response.put("price", entry.price)

        return response
    }

    override fun getTargetClass(): Class<Resource> {
        return Resource::class.java
    }
}
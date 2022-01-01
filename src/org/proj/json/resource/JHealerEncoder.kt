package org.proj.json.resource

import org.proj.game.Healer
import org.sjr.JSONObj
import org.sjr.codec.JSONEncoder

object JHealerEncoder: JSONEncoder<Healer> {
    override fun encode(p0: Healer): JSONObj {
        var json = JSONObj()
        json.put("health", p0.health.toInt())

        return json
    }

    override fun getTargetClass(): Class<Healer> {
        return Healer::class.java
    }
}
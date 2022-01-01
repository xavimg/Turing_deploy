package org.proj.json.resource

import org.proj.game.Damager
import org.proj.game.Healer
import org.proj.game.Resource
import org.proj.json.primitive.MapCodec
import org.sjr.JSONObj
import org.sjr.codec.JSONEncoder

object JDamagerEncoder: JSONEncoder<Damager> {
    private object RequisiteEncoder: JSONEncoder<Pair<Resource, UInt>> {
        override fun encode (p0: Pair<Resource, UInt>): JSONObj {
            var json = JSONObj()
            json.put("resource", p0.first.name)
            json.put("amount", p0.second.toInt())

            return json
        }

        override fun getTargetClass(): Class<Pair<Resource, UInt>> {
            val phantom : Pair<Resource?, UInt> = Pair(null, 0u)
            return phantom::class.java as Class<Pair<Resource, UInt>>
        }
    }

    override fun encode(p0: Damager): JSONObj {
        var json = JSONObj()
        json.put("requisite", RequisiteEncoder, p0.requisite)
        json.put("damage", p0.damage.toInt())

        return json
    }

    override fun getTargetClass(): Class<Damager> {
        return Damager::class.java
    }
}
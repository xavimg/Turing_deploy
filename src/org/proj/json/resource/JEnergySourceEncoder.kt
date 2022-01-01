package org.proj.json.resource

import org.proj.game.Damager
import org.proj.game.EnergySource
import org.sjr.JSONObj
import org.sjr.codec.JSONEncoder

object JEnergySourceEncoder: JSONEncoder<EnergySource> {
    override fun encode(p0: EnergySource): JSONObj {
        var json = JSONObj()
        json.put("energy", p0.energy.toInt())

        return json
    }

    override fun getTargetClass(): Class<EnergySource> {
        return EnergySource::class.java
    }
}
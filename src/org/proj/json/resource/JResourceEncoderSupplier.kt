package org.proj.json.resource

import org.proj.game.*
import org.sjr.JSONObj
import org.sjr.codec.JSONEncoder
import org.sjr.supplier.JSONEncoderSupplier
import java.util.*
import kotlin.collections.ArrayList

object JResourceEncoderSupplier: JSONEncoderSupplier {
    private class EncoderConcat (val encoders: List<JSONEncoder<Resource>>): JSONEncoder<Resource> {
        override fun encode(p0: Resource?): JSONObj {
            val resp = JSONObj();
            for (encoder in this.encoders) {
                resp.put(encoder.targetClass.simpleName.lowercase(), encoder.encode(p0))
            }

            return resp
        }

        override fun getTargetClass(): Class<Resource> {
            return Resource::class.java
        }
    }

    override fun <T : Any?> encoder (p0: Class<T>?): Optional<out JSONEncoder<T>> {
        if (!Resource::class.java.isAssignableFrom(p0)) {
            return Optional.empty();
        }

        var result = ArrayList<JSONEncoder<Resource>>();
        result.add(JResourceEncoder)

        if (Composite::class.java.isAssignableFrom(p0)) {
            result.add(JCompositeEncoder as JSONEncoder<Resource>)
        }

        if (EnergySource::class.java.isAssignableFrom(p0)) {
            result.add(JEnergySourceEncoder as JSONEncoder<Resource>)
        }

        if (Healer::class.java.isAssignableFrom(p0)) {
            result.add(JHealerEncoder as JSONEncoder<Resource>)
        }

        if (Damager::class.java.isAssignableFrom(p0)) {
            result.add(JDamagerEncoder as JSONEncoder<Resource>)
        }

        return Optional.of(EncoderConcat(result) as JSONEncoder<T>)
    }
}
package org.proj.json.resource

import org.proj.game.Composite
import org.proj.game.Resource
import org.sjr.JSONObj
import org.sjr.codec.JSONEncoder

object JCompositeEncoder: JSONEncoder<Composite> {
    private object ParentEncoder: JSONEncoder<Map.Entry<Resource, UInt>> {
        object PhantomEntry: Map.Entry<Resource?, UInt> {
            override val key: Resource? = null
            override val value: UInt = 0u
        }

        override fun encode (p0: Map.Entry<Resource, UInt>): JSONObj {
            var json = JSONObj()
            json.put("resource", p0.key.name)
            json.put("amount", p0.value.toInt())

            return json
        }

        override fun getTargetClass(): Class<Map.Entry<Resource, UInt>> {
            return PhantomEntry::class.java as Class<Map.Entry<Resource, UInt>>
        }
    }

    override fun encode (p0: Composite): JSONObj {
        val json = JSONObj()
        val test = json.put("parents", ParentEncoder, *p0.parents.entries.toTypedArray())
        if (test.isPresent) {
            throw test.get()
        }

        return json
    }

    override fun getTargetClass(): Class<Composite> {
        return Composite::class.java;
    }
}
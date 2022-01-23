package org.proj.db.codecs

import org.bson.BsonReader
import org.bson.BsonType
import org.bson.BsonWriter
import org.bson.codecs.Codec
import org.bson.codecs.DecoderContext
import org.bson.codecs.EncoderContext
import org.proj.db.PlayerId
import org.proj.game.Inventory
import org.proj.game.Player
import org.proj.game.PlayerResource
import org.proj.game.Resource
import kotlin.test.assertNotNull

object PlayerCodec: Codec<Player> {
    override fun encode(writer: BsonWriter?, value: Player?, encoderContext: EncoderContext?) {
        assertNotNull(writer)
        assertNotNull(value)

        writer.writeStartDocument()
        writer.writeObjectId("_id", value.id.game)
        writer.writeInt32("website_id", value.id.website)
        writer.writeDouble("health", value.health)

        writer.writeStartDocument("inventory")
        writer.writeDouble("max_capacity", value.inventory.maxCapacity)

        writer.writeStartArray()
        for (entry in value.inventory.entries) {
            writer.writeStartDocument("resource")
            writer.writeString("name", entry.key.resource.name)
            writer.writeDouble("life_left", entry.key.lifeLeft)
            writer.writeInt32("amount", entry.value.toInt())
            writer.writeEndDocument()
        }
        writer.writeEndArray()

        writer.writeEndDocument()
        writer.writeEndDocument()
    }

    override fun getEncoderClass(): Class<Player> {
        return Player::class.java
    }

    override fun decode(reader: BsonReader?, decoderContext: DecoderContext?): Player {
        assertNotNull(reader)
        val id = PlayerId(game = reader.readObjectId("_id"), website = reader.readInt32("website_id"))
        val health = reader.readDouble("health")

        reader.readName("inventory")
        reader.readStartDocument()
        val maxCapacity = reader.readDouble("max_capacity")
        val resources = HashMap<PlayerResource, UInt>();

        reader.readStartArray()
        while (reader.currentBsonType != BsonType.END_OF_DOCUMENT) {
            reader.readName("resource")
            reader.readStartDocument()

            val resource = Resource.valueOf(reader.readString("name"))!!
            val lifeLeft = reader.readDouble("life_left")

            resources[PlayerResource(id, resource, lifeLeft)] = reader.readInt32("amount").toUInt()
            reader.readEndDocument()
        }
        reader.readEndArray()

        return Player(id = id, health = health, inventory = Inventory(id, maxCapacity, resources))
    }
}
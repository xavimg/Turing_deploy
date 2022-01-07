package org.proj.db

import org.bson.codecs.configuration.CodecRegistry
import org.bson.codecs.configuration.CodecRegistries
import org.proj.db.codecs.primitive.PrimitiveProvider
import com.mongodb.client.MongoDatabase
import com.mongodb.client.MongoCollection
import org.proj.game.PlanetarySystem
import io.github.cdimascio.dotenv.Dotenv
import com.mongodb.client.MongoClients
import org.bson.BsonDocument
import org.bson.BsonElement
import org.bson.BsonInt64
import org.bson.codecs.Codec
import org.proj.db.codecs.PlanetarySystemCodec

object Database {
    private val primitives = CodecRegistries.fromProviders(PrimitiveProvider.INSTANCE)
    val client : MongoDatabase
    val systems : MongoCollection<PlanetarySystem>

    init {
        val env = Dotenv.configure().load()
        val username = env["TURING_USERNAME"]!!
        val database = env["TURING_DATABASE"]!!
        val password = env["TURING_PASSWORD"]!!

        val client = MongoClients.create("mongodb://$username:$password@127.0.0.1:27017/?authSource=admin&readPreference=primary&directConnection=true&ssl=false")
        this.client = client.getDatabase(database)
        this.systems = this.client.getCollection("system", PlanetarySystem::class.java)
            .withCodecRegistry(getRegistry(PlanetarySystemCodec.INSTANCE))
    }

    fun ping (): Boolean {
        val response = this.client.runCommand(BsonDocument("ping", BsonInt64(1)))
        return response.getDouble("ok") == 1.0
    }

    private fun getRegistry(codec: Codec<*>): CodecRegistry {
        return CodecRegistries.fromRegistries(primitives, CodecRegistries.fromCodecs(codec))
    }
}
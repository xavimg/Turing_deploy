package org.proj.db;

import com.mongodb.MongoClientSettings;
import com.mongodb.MongoCredential;
import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoCollection;
import com.mongodb.client.MongoDatabase;
import org.bson.codecs.Codec;
import org.bson.codecs.configuration.CodecRegistries;
import org.bson.codecs.configuration.CodecRegistry;
import org.proj.db.codecs.PlanetarySystemCodec;
import org.proj.db.codecs.primitive.PrimitiveProvider;
import org.proj.game.PlanetarySystem;

public class Database {
    final private static CodecRegistry PRIMITIVES = CodecRegistries.fromProviders(PrimitiveProvider.INSTANCE);

    final public static MongoDatabase DB = initialize();
    final public static MongoCollection<PlanetarySystem> SYSTEMS = initializeSystems();

    private static MongoDatabase initialize () {
        String username = System.getenv("TURING_USERNAME");
        String database = System.getenv("TURING_DATABASE");
        String password = System.getenv("TURING_PASSWORD");

        MongoCredential credential = MongoCredential.createCredential(username, database, password.toCharArray());
        MongoClientSettings settings = MongoClientSettings.builder().credential(credential).build();
        MongoClient client = MongoClients.create(settings);

        return client.getDatabase(database);
    }

    private static CodecRegistry getRegistry (Codec codec) {
        return CodecRegistries.fromRegistries(PRIMITIVES, CodecRegistries.fromCodecs(codec));
    }

    private static MongoCollection<PlanetarySystem> initializeSystems () {
        return DB.getCollection("system", PlanetarySystem.class)
                .withCodecRegistry(getRegistry(PlanetarySystemCodec.INSTANCE));
    }
}

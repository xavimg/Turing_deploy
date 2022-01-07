package org.proj.db;

import com.mongodb.MongoClientSettings;
import com.mongodb.MongoCredential;
import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoCollection;
import com.mongodb.client.MongoDatabase;
import io.github.cdimascio.dotenv.Dotenv;
import org.bson.BsonDocument;
import org.bson.BsonInt64;
import org.bson.codecs.Codec;
import org.bson.codecs.configuration.CodecRegistries;
import org.bson.codecs.configuration.CodecRegistry;
import org.proj.db.codecs.PlanetarySystemCodec;
import org.proj.db.codecs.primitive.PrimitiveProvider;
import org.proj.game.PlanetarySystem;

import java.util.Objects;

public class Database {
    final private static CodecRegistry PRIMITIVES = CodecRegistries.fromProviders(PrimitiveProvider.INSTANCE);

    final public static MongoDatabase DB = initialize();
    final public static MongoCollection<PlanetarySystem> SYSTEMS = initializeSystems();

    public static void forceInit () {
        var a = DB;
    }

    private static MongoDatabase initialize () {
        Dotenv env = Dotenv.configure().load();

        String username = env.get("TURING_USERNAME");
        String database = env.get("TURING_DATABASE");
        String password = env.get("TURING_PASSWORD");

        Objects.requireNonNull(username, "No username found");
        Objects.requireNonNull(database, "No database found");
        Objects.requireNonNull(password, "No password found");

        MongoCredential credential = MongoCredential.createCredential(username, database, password.toCharArray());
        MongoClientSettings settings = MongoClientSettings.builder().credential(credential).build();
        MongoClient client = MongoClients.create(settings);

        MongoDatabase db = client.getDatabase(database);
        try {
            db.runCommand(new BsonDocument("ping", new BsonInt64(1)));
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }

        return db;
    }

    private static CodecRegistry getRegistry (Codec codec) {
        return CodecRegistries.fromRegistries(PRIMITIVES, CodecRegistries.fromCodecs(codec));
    }

    private static MongoCollection<PlanetarySystem> initializeSystems () {
        return DB.getCollection("system", PlanetarySystem.class)
                .withCodecRegistry(getRegistry(PlanetarySystemCodec.INSTANCE));
    }
}

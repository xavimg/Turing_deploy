package org.proj.data.cross.write;

import org.bson.BsonWriter;
import org.bson.codecs.EncoderContext;
import org.proj.data.json.JSONObject;

import java.util.ArrayList;
import java.util.Optional;

public interface CrossWriter {
    void writeKey (String name);
    void writeNull ();
    void writeUndefined ();

    void writeString (String value);
    void writeSymbol (String value);

    void writeBool (boolean value);
    void writeInt (int value);
    void writeLong (long value);
    void writeFloat (float value);
    void writeDouble (double value);

    void writeStartArray ();
    void writeStartDocument ();

    void writeEndArray ();
    void writeEndDocument();

    default void writeNull (String key) {
        this.writeKey(key);
        this.writeNull();
    }

    default void writeUndefined (String key) {
        this.writeKey(key);
        this.writeUndefined();
    }

    default void writeString (String key, String value) {
        this.writeKey(key);
        this.writeString(value);
    }

    default void writeSymbol (String key, String value) {
        this.writeKey(key);
        this.writeSymbol(value);
    }

    default void writeBool (String key, boolean value) {
        this.writeKey(key);
        this.writeBool(value);
    }

    default void writeInt (String key, int value) {
        this.writeKey(key);
        this.writeInt(value);
    }

    default void writeLong (String key, long value) {
        this.writeKey(key);
        this.writeLong(value);
    }

    default void writeFloat (String key, float value) {
        this.writeKey(key);
        this.writeFloat(value);
    }

    default void writeDouble (String key, double value) {
        this.writeKey(key);
        this.writeDouble(value);
    }

    default void writeStartArray (String key) {
        this.writeKey(key);
        this.writeStartArray();
    }

    default void writeStartDocument (String key) {
        this.writeKey(key);
        this.writeStartDocument();
    }

    static CrossWriter fromBson (BsonWriter writer, EncoderContext context) {
        return new BsonWrapper(writer, context);
    }

    /*static CrossWriter fromJson (JSONObject writer) {
        return new JsonWrapper(writer);
    }*/

    // SUBCLASSES
    class BsonWrapper implements CrossWriter {
        final private BsonWriter parent;
        final private EncoderContext context;

        public BsonWrapper (BsonWriter parent, EncoderContext context) {
            this.parent = parent;
            this.context = context;
        }

        @Override
        public void writeKey (String name) {
            this.parent.writeName(name);
        }

        @Override
        public void writeNull() {
            this.parent.writeNull();
        }

        @Override
        public void writeUndefined() {
            this.parent.writeUndefined();
        }

        @Override
        public void writeString(String value) {
            this.parent.writeString(value);
        }

        @Override
        public void writeSymbol(String value) {
            this.parent.writeSymbol(value);
        }

        @Override
        public void writeBool(boolean value) {
            this.parent.writeBoolean(value);
        }

        @Override
        public void writeInt(int value) {
            this.parent.writeInt32(value);
        }

        @Override
        public void writeLong(long value) {
            this.parent.writeInt64(value);
        }

        @Override
        public void writeFloat(float value) {
            this.parent.writeDouble(value);
        }

        @Override
        public void writeDouble(double value) {
            this.parent.writeDouble(value);
        }

        @Override
        public void writeStartArray() {
            this.parent.writeStartArray();
        }

        @Override
        public void writeStartDocument() {
            this.parent.writeStartDocument();
        }

        @Override
        public void writeEndArray() {
            this.parent.writeEndArray();
        }

        @Override
        public void writeEndDocument() {
            this.parent.writeEndDocument();
        }
    }

    /*
    class JsonWrapper implements CrossWriter {
        final private JSONObject parent;
        private Optional<String> cacheKey;

        private Optional<ArrayList<Object>> array;

        public JsonWrapper (JSONObject parent) {
            this.parent = parent;
            this.cacheKey = Optional.empty();
            this.array = Optional.empty();
        }

        @Override
        public void writeKey (String name) {
            if (cacheKey.isPresent()) {
                throw new UnsupportedOperationException();
            }

            cacheKey = Optional.of(name);
        }

        @Override
        public void writeNull() {
            String key = cacheKey.get();
            cacheKey = Optional.empty();
            this.parent.putNull(key);
        }

        @Override
        public void writeUndefined() {
            writeNull();
        }

        @Override
        public void writeString(String value) {
            String key = cacheKey.get();
            cacheKey = Optional.empty();
            this.parent.put(key, value);
        }

        @Override
        public void writeSymbol(String value) {
            writeString(value);
        }

        @Override
        public void writeBool(boolean value) {
            String key = cacheKey.get();
            cacheKey = Optional.empty();
            this.parent.put(key, value);
        }

        @Override
        public void writeInt(int value) {
            String key = cacheKey.get();
            cacheKey = Optional.empty();
            this.parent.put(key, value);
        }

        @Override
        public void writeLong(long value) {
            String key = cacheKey.get();
            cacheKey = Optional.empty();
            this.parent.put(key, value);
        }

        @Override
        public void writeFloat(float value) {
            String key = cacheKey.get();
            cacheKey = Optional.empty();
            this.parent.put(key, value);
        }

        @Override
        public void writeDouble(double value) {
            String key = cacheKey.get();
            cacheKey = Optional.empty();
            this.parent.put(key, value);
        }

        @Override
        public void writeStartArray() {
            this.array = Optional.of(new ArrayList<>());
        }

        @Override
        public void writeStartDocument() {
            this.parent.put();
        }

        @Override
        public void writeEndArray() {
            this.parent.endArray();
        }

        @Override
        public void writeEndDocument() {
            this.parent.endObject();
        }
    }*/
}

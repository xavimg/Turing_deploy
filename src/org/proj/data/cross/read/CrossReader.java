package org.proj.data.cross.read;

import org.bson.BsonReader;
import org.bson.codecs.DecoderContext;

public interface CrossReader {
    void readKey (String name);
    ValueType nextValueType ();

    void readNull ();
    void readUndefined ();

    String readString ();
    String readSymbol ();

    boolean readBool ();
    int readInt ();
    long readLong ();
    float readFloat ();
    double readDouble ();

    void readStartArray ();
    void readStartDocument ();

    void readEndArray ();
    void readEndDocument();

    void skipKey ();
    void skipValue ();

    default void readNull (String key) {
        this.readKey(key);
        this.readNull();
    }

    default void readUndefined (String key) {
        this.readKey(key);
        this.readUndefined();
    }

    default String readString (String key) {
        this.readKey(key);
        return this.readString();
    }

    default String readSymbol (String key) {
        this.readKey(key);
        return this.readSymbol();
    }

    default boolean readBool (String key) {
        this.readKey(key);
        return this.readBool();
    }

    default int readInt (String key) {
        this.readKey(key);
        return this.readInt();
    }

    default long readLong (String key) {
        this.readKey(key);
        return this.readLong();
    }

    default float readFloat (String key) {
        this.readKey(key);
        return this.readFloat();
    }

    default double readDouble (String key) {
        this.readKey(key);
        return this.readDouble();
    }

    default void readStartArray (String key) {
        this.readKey(key);
        this.readStartArray();
    }

    default void readStartDocument (String key) {
        this.readKey(key);
        this.readStartDocument();
    }

    static CrossReader fromBson (BsonReader reader, DecoderContext context) {
        return new BsonWrapper(reader, context);
    }

    /*static CrossReader fromJson (JSONReader reader) {
        return new JsonWrapper(reader);
    }*/

    // SUBCLASSES
    class BsonWrapper implements CrossReader {
        final private BsonReader parent;
        final private DecoderContext context;

        public BsonWrapper(BsonReader parent, DecoderContext context) {
            this.parent = parent;
            this.context = context;
        }

        @Override
        public void readKey (String name) {
            this.parent.readName(name);
        }

        @Override
        public ValueType nextValueType() {
            return switch (this.parent.getCurrentBsonType()) {
                case NULL -> ValueType.NULL;
                case UNDEFINED -> ValueType.UNDEFINED;
                case BOOLEAN -> ValueType.BOOL;
                case INT32 -> ValueType.INT;
                case INT64 -> ValueType.LONG;
                case DOUBLE -> ValueType.DOUBLE;
                case STRING -> ValueType.STRING;
                case SYMBOL -> ValueType.SYMBOL;
                case ARRAY -> ValueType.ARRAY;
                case DOCUMENT -> ValueType.DOCUMENT;
                case END_OF_DOCUMENT -> ValueType.END;
                default -> {throw new RuntimeException();}
            };
        }

        @Override
        public void readNull() {
            this.parent.readNull();
        }

        @Override
        public void readUndefined() {
            this.parent.readUndefined();
        }

        @Override
        public String readString() {
            return this.parent.readString();
        }

        @Override
        public String readSymbol() {
            return this.parent.readSymbol();
        }

        @Override
        public boolean readBool() {
            return this.parent.readBoolean();
        }

        @Override
        public int readInt() {
            return this.parent.readInt32();
        }

        @Override
        public long readLong() {
            return this.parent.readInt64();
        }

        @Override
        public float readFloat() {
            return (float) this.parent.readDouble();
        }

        @Override
        public double readDouble() {
            return this.parent.readDouble();
        }

        @Override
        public void readStartArray() {
            this.parent.readStartArray();
        }

        @Override
        public void readStartDocument() {
            this.parent.readStartDocument();
        }

        @Override
        public void readEndArray() {
            this.parent.readEndArray();
        }

        @Override
        public void readEndDocument() {
            this.parent.readEndDocument();
        }

        @Override
        public void skipKey() {
            this.parent.skipName();
        }

        @Override
        public void skipValue() {
            this.parent.skipValue();
        }
    }

    /*
    class JsonWrapper implements CrossReader {
        final private JSONReader parent;

        public JsonWrapper(JSONReader parent) {
            this.parent = parent;
        }

        @Override
        public void readKey(String name) {
            this.parent.readKey(name);
        }

        @Override
        public ValueType nextValueType() {
            return switch (this.parent.getNextType()) {
                case NULL -> ValueType.NULL;
                case BOOL -> ValueType.BOOL;
                case NUMBER -> ValueType.DOUBLE;
                case STRING -> ValueType.STRING;
                case ARRAY -> ValueType.ARRAY;
                case OBJECT -> ValueType.DOCUMENT;
                case END -> ValueType.END;
                default -> {throw new RuntimeException();}
            };
        }

        @Override
        public void readNull() {
            this.parent.readNull();
        }

        @Override
        public void readUndefined() {
            this.parent.readNull();
        }

        @Override
        public String readString() {
            return this.parent.readString();
        }

        @Override
        public String readSymbol() {
            return this.parent.readString();
        }

        @Override
        public boolean readBool() {
            return this.parent.readBool();
        }

        @Override
        public int readInt() {
            return this.parent.readInt();
        }

        @Override
        public long readLong() {
            return this.parent.readLong();
        }

        @Override
        public float readFloat() {
            return this.parent.readFloat();
        }

        @Override
        public double readDouble() {
            return this.parent.readDouble();
        }

        @Override
        public void readStartArray() {
            this.parent.startArray();
        }

        @Override
        public void readStartDocument() {
            this.parent.startObject();
        }

        @Override
        public void readEndArray() {
            this.parent.end();
        }

        @Override
        public void readEndDocument() {
            this.parent.end();
        }

        @Override
        public void skipKey() {
            this.parent.skipKey();
        }

        @Override
        public void skipValue() {
            this.parent.skipValue();
        }
    }*/
}

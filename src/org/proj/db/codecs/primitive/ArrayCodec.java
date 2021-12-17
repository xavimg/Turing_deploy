package org.proj.db.codecs.primitive;

import org.bson.BsonReader;
import org.bson.BsonType;
import org.bson.BsonWriter;
import org.bson.codecs.Codec;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;
import org.bson.types.Code;

import java.lang.reflect.Array;
import java.util.ArrayList;

public class ArrayCodec<T> implements Codec<T[]> {
    final private Codec<T> codec;

    public ArrayCodec (Codec<T> codec) {
        this.codec = codec;
    }

    @Override
    public T[] decode (BsonReader reader, DecoderContext context) {
        ArrayList<T> list = new ArrayList<>();

        reader.readStartArray();
        while (reader.getCurrentBsonType() != BsonType.END_OF_DOCUMENT) {
            list.add(codec.decode(reader, context));
        }
        reader.readEndArray();

        T[] target = (T[]) Array.newInstance(codec.getEncoderClass(), list.size());
        return list.toArray(target);
    }

    @Override
    public void encode (BsonWriter writer, T[] value, EncoderContext context) {
        writer.writeStartArray();
        for (T elem: value) {
            codec.encode(writer, elem, context);
        }
        writer.writeEndArray();
    }

    @Override
    public Class<T[]> getEncoderClass() {
        T[] dummy = (T[]) Array.newInstance(codec.getEncoderClass(), 0);
        return (Class<T[]>) dummy.getClass();
    }
}

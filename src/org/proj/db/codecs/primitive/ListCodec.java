package org.proj.db.codecs.primitive;

import org.bson.BsonReader;
import org.bson.BsonType;
import org.bson.BsonWriter;
import org.bson.codecs.Codec;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;

import java.util.ArrayList;
import java.util.List;

public class ListCodec<T> implements Codec<List<T>> {
    final private Codec<T> codec;

    public ListCodec (Codec<T> codec) {
        this.codec = codec;
    }

    @Override
    public List<T> decode (BsonReader reader, DecoderContext decoderContext) {
        ArrayList<T> list = new ArrayList<>();
        reader.readStartArray();

        while (reader.getCurrentBsonType() != BsonType.END_OF_DOCUMENT) {
            list.add(codec.decode(reader, decoderContext));
        }

        reader.readEndArray();
        return list;
    }

    @Override
    public void encode (BsonWriter writer, List<T> value, EncoderContext encoderContext) {
        writer.writeStartArray();
        for (T elem: value) {
            codec.encode(writer, elem, encoderContext);
        }

        writer.writeEndArray();
    }

    @Override
    public Class<List<T>> getEncoderClass() {
        List<T> dummy = new ArrayList<>();
        return (Class<List<T>>) dummy.getClass();
    }
}

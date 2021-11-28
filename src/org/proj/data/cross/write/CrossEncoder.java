package org.proj.data.cross.write;

import org.bson.BsonWriter;
import org.bson.codecs.Encoder;
import org.bson.codecs.EncoderContext;
import org.proj.data.json.codec.JSONEncoder;

public interface CrossEncoder<T> extends Encoder<T>, JSONEncoder<T> {
    void encode (CrossWriter writer, T value);

    @Override
    default void encode (BsonWriter writer, T value, EncoderContext encoderContext) {
        this.encode(CrossWriter.fromBson(writer, encoderContext), value);
    }

    /*@Override
    default void encode(JSONWriter writer, T value) {
        this.encode(CrossWriter.fromJson(writer), value);
    }*/
}

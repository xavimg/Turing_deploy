package org.proj.data.cross.read;

import org.bson.BsonReader;
import org.bson.codecs.Decoder;
import org.bson.codecs.DecoderContext;
import org.proj.data.json.codec.JSONDecoder;

public interface CrossDecoder<T> extends Decoder<T>, JSONDecoder<T> {
    T decode (CrossReader reader);

    @Override
    default T decode(BsonReader reader, DecoderContext decoderContext) {
        return this.decode(CrossReader.fromBson(reader, decoderContext));
    }

    /*@Override
    default T decode(JSONReader reader) {
        return this.decode(CrossReader.fromJson(reader));
    }*/
}

package org.proj.db.codecs;

import org.bson.BsonReader;
import org.bson.BsonWriter;
import org.bson.codecs.Codec;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;
import org.proj.db.codecs.pseudo.NullCodec;
import org.proj.math.vector.Vec2;

public class TwoVectorCodec implements Codec<Vec2> {
    final public static TwoVectorCodec INSTANCE = new TwoVectorCodec();
    private TwoVectorCodec () {}

    @Override
    public Vec2 decode (BsonReader reader, DecoderContext decoderContext) {
        if (NullCodec.decode(reader)) return null;

        reader.readStartDocument();
        double x = reader.readDouble("x");
        double y = reader.readDouble("y");
        reader.readEndDocument();

        return new Vec2(x, y);
    }

    @Override
    public void encode (BsonWriter writer, Vec2 value, EncoderContext encoderContext) {
        if (NullCodec.encode(writer, value)) return;

        writer.writeStartDocument();
        writer.writeDouble("x", value.x);
        writer.writeDouble("y", value.y);
        writer.writeEndDocument();
    }

    @Override
    public Class<Vec2> getEncoderClass() {
        return Vec2.class;
    }
}

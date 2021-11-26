package org.proj.db.codecs;

import org.bson.BsonReader;
import org.bson.BsonWriter;
import org.bson.codecs.Codec;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;
import org.proj.db.codecs.pseudo.SpaceBodyCodec;
import org.proj.game.SpaceBody;
import org.proj.game.Sun;

public class SunCodec implements Codec<Sun> {
    final public static SunCodec INSTANCE = new SunCodec();
    private SunCodec () {}

    @Override
    public Sun decode (BsonReader reader, DecoderContext decoderContext) {
        reader.readStartDocument();

        SpaceBody decode = SpaceBodyCodec.decode(reader, decoderContext);
        double temp = reader.readDouble("temperature");

        reader.readEndDocument();
        return new Sun(decode.restMass(), decode.radius(), decode.getPosition(), decode.getVelocity(), decode.color, null, temp);
    }

    @Override
    public void encode (BsonWriter writer, Sun value, EncoderContext encoderContext) {
        writer.writeStartDocument();
        SpaceBodyCodec.encode(writer, value, encoderContext);
        writer.writeDouble("temperature", value.temperature);
        writer.writeEndDocument();
    }

    @Override
    public Class<Sun> getEncoderClass() {
        return Sun.class;
    }
}

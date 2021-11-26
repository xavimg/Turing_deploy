package org.proj.db.codecs;

import org.bson.BsonReader;
import org.bson.BsonWriter;
import org.bson.codecs.Codec;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;
import org.proj.game.PlanetarySystem;
import org.proj.game.Sun;

public class PlanetarySystemCodec implements Codec<PlanetarySystem> {
    final public static PlanetarySystemCodec INSTANCE = new PlanetarySystemCodec();
    private PlanetarySystemCodec () {}

    @Override
    public PlanetarySystem decode (BsonReader reader, DecoderContext decoderContext) {
        return null;
    }

    @Override
    public void encode (BsonWriter writer, PlanetarySystem value, EncoderContext encoderContext) {
        writer.writeStartDocument();
        writer.writeStartArray("suns");

        for (Sun sun: value.getSuns()) {
            SunCodec.INSTANCE.encode(writer, sun, encoderContext);
        }

        writer.writeEndArray();
        writer.writeEndDocument();
    }

    @Override
    public Class<PlanetarySystem> getEncoderClass() {
        return PlanetarySystem.class;
    }
}

package org.proj.db.codecs;

import org.bson.BsonReader;
import org.bson.BsonWriter;
import org.bson.codecs.Codec;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;
import org.proj.db.codecs.pseudo.SpaceBodyCodec;
import org.proj.game.Planet;
import org.proj.game.SpaceBody;
import org.proj.game.Sun;

public class PlanetCodec implements Codec<Planet> {
    final public static PlanetCodec INSTANCE = new PlanetCodec();
    private PlanetCodec() {}

    @Override
    public Planet decode (BsonReader reader, DecoderContext decoderContext) {
        reader.readStartDocument();
        SpaceBody decode = SpaceBodyCodec.decode(reader, decoderContext);
        reader.readEndDocument();

        return new Planet(decode.restMass(), decode.radius(), decode.getPosition(), decode.getVelocity(), decode.color, null);
    }

    @Override
    public void encode (BsonWriter writer, Planet value, EncoderContext encoderContext) {
        writer.writeStartDocument();
        SpaceBodyCodec.encode(writer, value, encoderContext);
        writer.writeEndDocument();
    }

    @Override
    public Class<Planet> getEncoderClass() {
        return Planet.class;
    }
}

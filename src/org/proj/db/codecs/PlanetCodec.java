package org.proj.db.codecs;

import org.proj.data.cross.CrossCodec;
import org.proj.data.cross.read.CrossReader;
import org.proj.data.cross.write.CrossWriter;
import org.proj.db.codecs.pseudo.SpaceBodyCodec;
import org.proj.game.Planet;
import org.proj.game.SpaceBody;

public class PlanetCodec implements CrossCodec<Planet> {
    final public static PlanetCodec INSTANCE = new PlanetCodec();
    private PlanetCodec() {}

    @Override
    public Planet decode (CrossReader reader) {
        reader.readStartDocument();
        SpaceBody decode = SpaceBodyCodec.decode(reader);
        reader.readEndDocument();

        return new Planet(decode.restMass(), decode.radius(), decode.getPosition(), decode.getVelocity(), decode.color, null);
    }

    @Override
    public void encode (CrossWriter writer, Planet value) {
        writer.writeStartDocument();
        SpaceBodyCodec.encode(writer, value);
        writer.writeEndDocument();
    }

    @Override
    public Class<Planet> getEncoderClass() {
        return Planet.class;
    }
}

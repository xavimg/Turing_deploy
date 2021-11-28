package org.proj.db.codecs;

import org.proj.data.cross.CrossCodec;
import org.proj.data.cross.read.CrossReader;
import org.proj.data.cross.write.CrossWriter;
import org.proj.db.codecs.primitive.ArrayCodec;
import org.proj.game.Planet;
import org.proj.game.PlanetarySystem;
import org.proj.game.Sun;

public class PlanetarySystemCodec implements CrossCodec<PlanetarySystem> {
    final public static PlanetarySystemCodec INSTANCE = new PlanetarySystemCodec();

    final private ArrayCodec<Sun> SUN = new ArrayCodec<>(SunCodec.INSTANCE);
    final private ArrayCodec<Planet> PLANET = new ArrayCodec<>(PlanetCodec.INSTANCE);

    private PlanetarySystemCodec () {}

    @Override
    public PlanetarySystem decode (CrossReader reader) {
        reader.readStartDocument();
        /*reader.readBsonType();*/ reader.skipKey(); reader.skipValue();

        reader.readKey("suns");
        Sun[] suns = SUN.decode(reader);

        reader.readKey("planets");
        Planet[] planets = PLANET.decode(reader);

        reader.readEndDocument();
        return new PlanetarySystem(suns, planets);
    }

    @Override
    public void encode (CrossWriter writer, PlanetarySystem value) {
        writer.writeStartDocument();

        writer.writeKey("suns");
        SUN.encode(writer, value.getSuns());

        writer.writeKey("planets");
        PLANET.encode(writer, value.getPlanets());

        writer.writeEndDocument();
    }

    @Override
    public Class<PlanetarySystem> getEncoderClass() {
        return PlanetarySystem.class;
    }
}

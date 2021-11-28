package org.proj.db.codecs;

import org.proj.data.cross.CrossCodec;
import org.proj.data.cross.read.CrossReader;
import org.proj.data.cross.write.CrossWriter;
import org.proj.db.codecs.pseudo.SpaceBodyCodec;
import org.proj.game.SpaceBody;
import org.proj.game.Sun;

public class SunCodec implements CrossCodec<Sun> {
    final public static SunCodec INSTANCE = new SunCodec();
    private SunCodec () {}

    @Override
    public Sun decode (CrossReader reader) {
        reader.readStartDocument();

        SpaceBody decode = SpaceBodyCodec.decode(reader);
        double temp = reader.readDouble("temperature");

        reader.readEndDocument();
        return new Sun(decode.restMass(), decode.radius(), decode.getPosition(), decode.getVelocity(), decode.color, null, temp);
    }

    @Override
    public void encode (CrossWriter writer, Sun value) {
        writer.writeStartDocument();
        SpaceBodyCodec.encode(writer, value);
        writer.writeDouble("temperature", value.temperature);
        writer.writeEndDocument();
    }

    @Override
    public Class<Sun> getEncoderClass() {
        return Sun.class;
    }
}

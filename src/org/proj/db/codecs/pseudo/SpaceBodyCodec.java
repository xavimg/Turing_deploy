package org.proj.db.codecs.pseudo;

import org.proj.data.cross.read.CrossReader;
import org.proj.data.cross.write.CrossWriter;
import org.proj.db.codecs.TwoVectorCodec;
import org.proj.db.codecs.primitive.ColorCodec;
import org.proj.game.SpaceBody;
import org.proj.math.vector.Vec2;

import java.awt.*;

public class SpaceBodyCodec {
    public static SpaceBody decode (CrossReader reader) {
        double restMass = reader.readDouble("rest_mass");
        double radius = reader.readDouble("radius");

        reader.readKey("position");
        Vec2 position = TwoVectorCodec.INSTANCE.decode(reader);

        reader.readKey("velocity");
        Vec2 velocity = TwoVectorCodec.INSTANCE.decode(reader);

        reader.readKey("color");
        Color color = ColorCodec.INSTANCE.decode(reader);

        return new SpaceBody(restMass, radius, position, velocity, null, color, null);
    }

    public static void encode (CrossWriter writer, SpaceBody value) {
        writer.writeDouble("rest_mass", value.restMass());
        writer.writeDouble("radius", value.radius());

        writer.writeKey("position");
        TwoVectorCodec.INSTANCE.encode(writer, value.getPosition());

        writer.writeKey("velocity");
        TwoVectorCodec.INSTANCE.encode(writer, value.getVelocity());

        writer.writeKey("color");
        ColorCodec.INSTANCE.encode(writer, value.color);
    }
}

package org.proj.db.codecs.pseudo;

import org.bson.BsonReader;
import org.bson.BsonWriter;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;
import org.proj.db.codecs.TwoVectorCodec;
import org.proj.db.codecs.primitive.ColorCodec;
import org.proj.game.body.SpaceBody;
import org.proj.math.vector.Vec2;

import java.awt.*;

public class SpaceBodyCodec {
    public static SpaceBody decode (BsonReader reader, DecoderContext context) {
        double restMass = reader.readDouble("rest_mass");
        double radius = reader.readDouble("radius");

        reader.readName("position");
        Vec2 position = TwoVectorCodec.INSTANCE.decode(reader, context);

        reader.readName("velocity");
        Vec2 velocity = TwoVectorCodec.INSTANCE.decode(reader, context);

        reader.readName("color");
        Color color = ColorCodec.INSTANCE.decode(reader, context);

        return new SpaceBody(restMass, radius, position, velocity, null, color, null);
    }

    public static void encode (BsonWriter writer, SpaceBody value, EncoderContext context) {
        writer.writeDouble("rest_mass", value.restMass());
        writer.writeDouble("radius", value.radius());

        writer.writeName("position");
        TwoVectorCodec.INSTANCE.encode(writer, value.getPosition(), context);

        writer.writeName("velocity");
        TwoVectorCodec.INSTANCE.encode(writer, value.getVelocity(), context);

        writer.writeName("color");
        ColorCodec.INSTANCE.encode(writer, value.color, context);
    }
}

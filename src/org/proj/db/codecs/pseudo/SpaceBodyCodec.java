package org.proj.db.codecs.pseudo;

import org.bson.BsonReader;
import org.bson.BsonWriter;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;
import org.proj.db.codecs.TwoVectorCodec;
import org.proj.db.codecs.primitive.ColorCodec;
import org.proj.game.SpaceBody;
import org.proj.math.vector.Vec2;

import java.awt.*;

public class SpaceBodyCodec {
    public static SpaceBody decode (BsonReader reader, DecoderContext decoderContext) {
        double restMass = reader.readDouble("rest_mass");
        double radius = reader.readDouble("radius");

        reader.readName("position");
        Vec2 position = TwoVectorCodec.INSTANCE.decode(reader, decoderContext);

        reader.readName("velocity");
        Vec2 velocity = TwoVectorCodec.INSTANCE.decode(reader, decoderContext);

        reader.readName("color");
        Color color = ColorCodec.INSTANCE.decode(reader, decoderContext);

        return new SpaceBody(restMass, radius, position, velocity, null, color, null);
    }

    public static void encode (BsonWriter writer, SpaceBody value, EncoderContext encoderContext) {
        writer.writeDouble("rest_mass", value.restMass());
        writer.writeDouble("radius", value.radius());

        writer.writeName("position");
        TwoVectorCodec.INSTANCE.encode(writer, value.getPosition(), encoderContext);

        writer.writeName("velocity");
        TwoVectorCodec.INSTANCE.encode(writer, value.getVelocity(), encoderContext);

        writer.writeName("color");
        ColorCodec.INSTANCE.encode(writer, value.color, encoderContext);
    }
}

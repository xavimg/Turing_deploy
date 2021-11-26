package org.proj.db.codecs.pseudo;

import org.bson.BsonWriter;
import org.bson.codecs.EncoderContext;
import org.proj.db.codecs.TwoVectorCodec;
import org.proj.db.codecs.primitive.ColorCodec;
import org.proj.game.SpaceBody;

public class SpaceBodyCodec {
    /*
    public static SpaceBody decode (BsonReader reader, DecoderContext decoderContext) {
        return null;
    }*/

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

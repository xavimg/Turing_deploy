package org.proj.db.codecs.primitive;

import org.bson.BsonReader;
import org.bson.BsonWriter;
import org.bson.codecs.Codec;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;

import java.awt.*;

public class ColorCodec implements Codec<Color> {
    final public static ColorCodec INSTANCE = new ColorCodec();
    private ColorCodec () {}

    @Override
    public Color decode (BsonReader reader, DecoderContext decoderContext) {
        return new Color(reader.readInt32(), true);
    }

    @Override
    public void encode (BsonWriter writer, Color value, EncoderContext encoderContext) {
        writer.writeInt32(value.getRGB());
    }

    @Override
    public Class<Color> getEncoderClass() {
        return Color.class;
    }
}

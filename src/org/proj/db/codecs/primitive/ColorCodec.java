package org.proj.db.codecs.primitive;

import org.proj.data.cross.CrossCodec;
import org.proj.data.cross.read.CrossReader;
import org.proj.data.cross.write.CrossWriter;
import org.proj.db.codecs.pseudo.NullCodec;

import java.awt.*;

public class ColorCodec implements CrossCodec<Color> {
    final public static ColorCodec INSTANCE = new ColorCodec();
    private ColorCodec () {}

    @Override
    public Color decode (CrossReader reader) {
        if (NullCodec.decode(reader)) return null;
        return new Color(reader.readInt(), true);
    }

    @Override
    public void encode (CrossWriter writer, Color value) {
        if (NullCodec.encode(writer, value)) return;
        writer.writeInt(value.getRGB());
    }

    @Override
    public Class<Color> getEncoderClass() {
        return Color.class;
    }
}

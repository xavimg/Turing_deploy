package org.proj.db.codecs;

import org.proj.data.cross.CrossCodec;
import org.proj.data.cross.read.CrossReader;
import org.proj.data.cross.write.CrossWriter;
import org.proj.db.codecs.pseudo.NullCodec;
import org.proj.math.vector.Vec2;

public class TwoVectorCodec implements CrossCodec<Vec2> {
    final public static TwoVectorCodec INSTANCE = new TwoVectorCodec();
    private TwoVectorCodec () {}

    @Override
    public Vec2 decode (CrossReader reader) {
        if (NullCodec.decode(reader)) return null;

        reader.readStartDocument();
        double x = reader.readDouble("x");
        double y = reader.readDouble("y");
        reader.readEndDocument();

        return new Vec2(x, y);
    }

    @Override
    public void encode (CrossWriter writer, Vec2 value) {
        if (NullCodec.encode(writer, value)) return;

        writer.writeStartDocument();
        writer.writeDouble("x", value.x);
        writer.writeDouble("y", value.y);
        writer.writeEndDocument();
    }

    @Override
    public Class<Vec2> getEncoderClass() {
        return Vec2.class;
    }
}

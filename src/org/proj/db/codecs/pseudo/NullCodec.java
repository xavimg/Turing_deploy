package org.proj.db.codecs.pseudo;

import org.proj.data.cross.read.CrossReader;
import org.proj.data.cross.read.ValueType;
import org.proj.data.cross.write.CrossWriter;

public class NullCodec {
    public static boolean decode (CrossReader reader) {
        boolean res = reader.nextValueType() == ValueType.NULL;
        if (res) {
            reader.skipValue();
        }

        return res;
    }

    public static boolean encode (CrossWriter writer, Object value) {
        boolean res = value == null;
        if (res) {
            writer.writeNull();
        }

        return res;
    }
}

package org.proj.db.codecs.pseudo;

import org.bson.BsonReader;
import org.bson.BsonType;
import org.bson.BsonWriter;
import org.bson.codecs.DecoderContext;
import org.bson.codecs.EncoderContext;

public class NullCodec {
    public static boolean decode (BsonReader reader) {
        return reader.getCurrentBsonType() == BsonType.NULL;
    }

    public static boolean encode (BsonWriter writer, Object value) {
        boolean res = value == null;
        if (res) {
            writer.writeNull();
        }

        return res;
    }
}

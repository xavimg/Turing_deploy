package org.proj.db.codecs.pseudo;

import org.bson.BsonReader;
import org.bson.BsonType;
import org.bson.BsonWriter;

public class NullCodec {
    public static boolean decode (BsonReader reader) {
        boolean res = reader.getCurrentBsonType() == BsonType.END_OF_DOCUMENT;
        if (res) {
            reader.skipValue();
        }

        return res;
    }

    public static boolean encode (BsonWriter writer, Object value) {
        boolean res = value == null;
        if (res) {
            writer.writeNull();
        }

        return res;
    }
}

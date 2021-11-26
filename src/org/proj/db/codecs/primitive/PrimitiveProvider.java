package org.proj.db.codecs.primitive;

import org.bson.codecs.*;
import org.proj.db.codecs.providers.CodecListProvider;

public class PrimitiveProvider extends CodecListProvider {
    final public static BooleanCodec BOOL = new BooleanCodec();
    final public static ByteCodec BYTE = new ByteCodec();
    final public static ShortCodec SHORT = new ShortCodec();
    final public static CharacterCodec CHAR = new CharacterCodec();
    final public static IntegerCodec INT = new IntegerCodec();
    final public static LongCodec LONG = new LongCodec();
    final public static FloatCodec FLOAT = new FloatCodec();
    final public static DoubleCodec DOUBLE = new DoubleCodec();
    final public static BigDecimalCodec BIG_DECIMAL = new BigDecimalCodec();

    final public static ObjectIdCodec OBJECT_ID = new ObjectIdCodec();
    final public static StringCodec STRING = new StringCodec();

    final public static CodecListProvider INSTANCE = new CodecListProvider(BOOL, BYTE, SHORT, CHAR, INT, LONG, FLOAT, DOUBLE, BIG_DECIMAL, OBJECT_ID, STRING);
}

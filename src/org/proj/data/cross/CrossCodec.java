package org.proj.data.cross;

import org.bson.codecs.Codec;
import org.proj.data.cross.read.CrossDecoder;
import org.proj.data.cross.write.CrossEncoder;
import org.proj.data.json.codec.JSONCodec;

public interface CrossCodec<T> extends Codec<T>, JSONCodec<T>, CrossDecoder<T>, CrossEncoder<T> {}

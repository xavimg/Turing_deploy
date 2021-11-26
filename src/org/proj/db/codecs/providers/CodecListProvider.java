package org.proj.db.codecs.providers;

import org.bson.codecs.Codec;
import org.bson.codecs.configuration.CodecProvider;
import org.bson.codecs.configuration.CodecRegistry;
import org.proj.utils.Range;

public class CodecListProvider implements CodecProvider {
    final private Codec[] codecs;

    public CodecListProvider (Codec... codecs) {
        this.codecs = codecs;
    }

    @Override
    public <T> Codec<T> get (Class<T> clazz, CodecRegistry registry) {
        return (Codec<T>) Range.ofArray(codecs, true).filter(x -> clazz.isAssignableFrom(x.getEncoderClass())).findFirst().orElse(null);
    }
}

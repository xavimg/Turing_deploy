package org.proj.db.codecs.providers;

import org.bson.codecs.Codec;
import org.bson.codecs.configuration.CodecProvider;
import org.bson.codecs.configuration.CodecRegistry;

public class ConcatProvider implements CodecProvider {
    final private CodecProvider first, last;

    public ConcatProvider (CodecProvider first, CodecProvider last) {
        this.first = first;
        this.last = last;
    }

    @Override
    public <T> Codec<T> get (Class<T> clazz, CodecRegistry registry) {
        Codec<T> first = this.first.get(clazz, registry);
        return first == null ? this.last.get(clazz, registry) : first;
    }
}

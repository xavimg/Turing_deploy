package org.proj.json.codec.primitive;

import org.json.simple.JSONObject;
import org.proj.json.JSONCodec;
import org.sjr.JSONObjectWrapper;

import java.util.HashMap;
import java.util.Map;
import java.util.function.Function;

public class MapCodec<K,V> implements JSONCodec<Map<K,V>> {
    final private Function<K,String> keyEncoder;
    final private Function<String,K> keyDecoder;
    final private JSONCodec<V> valueCodec;

    public MapCodec (Function<K, String> keyEncoder, Function<String, K> keyDecoder, JSONCodec<V> valueCodec) {
        this.keyEncoder = keyEncoder;
        this.keyDecoder = keyDecoder;
        this.valueCodec = valueCodec;
    }

    public MapCodec (Function<String, K> keyDecoder, JSONCodec<V> valueCodec) {
        this(Object::toString, keyDecoder, valueCodec);
    }

    public MapCodec (JSONCodec<V> valueCodec) {
        this(Object::toString, x -> null, valueCodec);
    }

    @Override
    public Map<K, V> decode (JSONObjectWrapper json) {
        HashMap<K, V> map = new HashMap<>();
        for (Object _entry: json.object.entrySet()) {
            var entry = (Map.Entry) _entry;
            K key = keyDecoder.apply(entry.getKey().toString());
            V value = valueCodec.decode(new JSONObjectWrapper((JSONObject) entry.getValue()));
            map.put(key, value);
        }

        return map;
    }

    @Override
    public JSONObject encode(Map<K,V> value) {
        JSONObject json = new JSONObject();
        for (Map.Entry<K, V> entry : value.entrySet()) {
            String key = keyEncoder.apply(entry.getKey());
            JSONObject val = valueCodec.encode(entry.getValue());
            json.put(key, val);
        }

        return json;
    }
}

package org.proj.json.codec.primitive;

import org.proj.json.JSONObject;
import org.proj.json.JSONCodec;

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
    public Map<K, V> decode (JSONObject json) {
        HashMap<K, V> map = new HashMap<>();
        for (Map.Entry<String, Object> entry: json.entrySet()) {
            K key = keyDecoder.apply(entry.getKey());
            V value = valueCodec.decode((JSONObject) entry.getValue());
            map.put(key, value);
        }

        return map;
    }

    @Override
    public JSONObject encode(Map<K,V> value) {
        JSONObject json = new JSONObject();
        for (Map.Entry<K,V> entry: value.entrySet()) {
            String key = keyEncoder.apply(entry.getKey());
            JSONObject val = valueCodec.encode(entry.getValue());
            json.put(key, val);
        }

        return json;
    }

    @Override
    public Class<Map<K, V>> getTransformClass() {
        return (Class<Map<K, V>>) Map.of().getClass();
    }
}

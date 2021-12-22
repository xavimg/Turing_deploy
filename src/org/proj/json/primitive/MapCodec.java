package org.proj.json.primitive;

import org.json.simple.JSONObject;
import org.sjr.JSONObj;
import org.sjr.codec.JSONCodec;

import java.util.HashMap;
import java.util.Map;
import java.util.function.Function;

public class MapCodec<K,V> implements JSONCodec<Map<K,V>> {
    final private Function<K, String> keyEncoder;
    final private Function<String, K> keyDecoder;
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
    public Map<K, V> decode (JSONObj json) {
        HashMap<K, V> map = new HashMap<>();
        for (Map.Entry<String, Object> entry: json.entrySet()) {
            K key = keyDecoder.apply(entry.getKey());
            V value = valueCodec.decode(new JSONObj((JSONObject) entry.getValue()));
            map.put(key, value);
        }

        return map;
    }

    @Override
    public JSONObj encode(Map<K,V> value) {
        JSONObj json = new JSONObj();
        for (Map.Entry<K, V> entry : value.entrySet()) {
            String key = keyEncoder.apply(entry.getKey());
            JSONObj val = valueCodec.encode(entry.getValue());
            json.put(key, val);
        }

        return json;
    }

    @Override
    public Class<Map<K, V>> getTargetClass() {
        Map<K,V> phantom = Map.of();
        return (Class<Map<K, V>>) phantom.getClass();
    }
}

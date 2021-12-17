package org.proj.json;

import org.proj.json.JSONObject;

public interface JSONEncoder<T> {
    JSONObject encode (T value);

    default void put (JSONObject object, String name, T value) {
        object.put(name, encode(value));
    }
}

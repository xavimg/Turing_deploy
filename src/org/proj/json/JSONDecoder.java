package org.proj.json;

public interface JSONDecoder<T> {
    T decode (JSONObject json);
    Class<T> getTransformClass ();

    default T get (JSONObject json, String name) {
        return json.get(name, this);
    }
}

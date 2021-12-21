package org.proj.json;

import org.sjr.JSONObjectWrapper;

public interface JSONDecoder<T> {
    T decode (JSONObjectWrapper json);
}

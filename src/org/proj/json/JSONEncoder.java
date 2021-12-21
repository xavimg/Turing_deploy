package org.proj.json;

import org.json.simple.JSONObject;
import org.sjr.JSONObjectWrapper;

public interface JSONEncoder<T> {
    JSONObject encode (T value);
}

package org.proj.data.json;

import java.util.Optional;

public class JSONWriter {
    final private JSONObject json;
    final private Optional<String> cacheKey;

    public JSONWriter (JSONObject json) {
        this.json = json;
        this.cacheKey = Optional.empty();
    }
}

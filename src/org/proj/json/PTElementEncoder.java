package org.proj.json;

import org.json.simple.JSONObject;
import org.proj.game.resource.PTElement;
import org.sjr.JSONObj;
import org.sjr.codec.JSONEncoder;

public class PTElementEncoder implements JSONEncoder<PTElement> {
    final public static PTElementEncoder INSTANCE = new PTElementEncoder();
    private PTElementEncoder () {}

    @Override
    public JSONObj encode (PTElement value) {
        JSONObj resp = new JSONObj();
        resp.put("name", value.getName());

        return resp;
    }
}

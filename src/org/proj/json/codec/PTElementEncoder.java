package org.proj.json.codec;

import org.json.simple.JSONObject;
import org.proj.game.resource.PTElement;
import org.proj.json.JSONEncoder;

public class PTElementEncoder implements JSONEncoder<PTElement> {
    final public static PTElementEncoder INSTANCE = new PTElementEncoder();
    private PTElementEncoder () {}

    @Override
    public JSONObject encode (PTElement value) {
        JSONObject resp = new JSONObject();
        resp.put("name", value.getName());
        return null;
    }
}

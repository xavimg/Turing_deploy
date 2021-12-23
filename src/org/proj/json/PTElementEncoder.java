package org.proj.json;

import org.proj.game.resource.PTElement;
import org.sjr.JSONObj;
import org.sjr.codec.JSONEncoder;

public class PTElementEncoder implements JSONEncoder<PTElement> {
    final public static PTElementEncoder INSTANCE = new PTElementEncoder();
    private PTElementEncoder () {}

    @Override
    public JSONObj encode (PTElement value) {
        JSONObj resp = new JSONObj();

        resp.put("number", value.getNumber());
        resp.put("name", value.getName());
        resp.put("mass", value.getMass());

        return resp;
    }
}

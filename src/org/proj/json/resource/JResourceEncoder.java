package org.proj.json.resource;

import org.proj.game.Resource;
import org.sjr.JSONObj;
import org.sjr.codec.JSONEncoder;

public class JResourceEncoder implements JSONEncoder<Resource> {
    final public static JResourceEncoder INSTANCE = new JResourceEncoder();
    private JResourceEncoder() {}

    @Override
    public JSONObj encode (Resource resource) {
        JSONObj resp = new JSONObj();
        resp.put("name", resource.getName());

        return resp;
    }
}

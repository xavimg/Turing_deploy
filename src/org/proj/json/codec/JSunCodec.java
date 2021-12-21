package org.proj.json.codec;

import org.json.simple.JSONObject;
import org.proj.game.body.SpaceBody;
import org.proj.game.body.Sun;
import org.proj.json.JSONCodec;
import org.sjr.JSONObjectWrapper;

public class JSunCodec implements JSONCodec<Sun> {
    final public static JSunCodec INSTANCE = new JSunCodec();
    private JSunCodec () {}

    @Override
    public JSONObject encode (Sun value) {
        var encode = JSpaceBodyCodec.INSTANCE.encode(value);
        encode.put("temperature", value.temperature);

        return encode;
    }

    @Override
    public Sun decode (JSONObjectWrapper json) {
        SpaceBody body = JSpaceBodyCodec.INSTANCE.decode(json);
        double temperature = json.getDouble("temperature").getAsDouble();

        return new Sun(body.restMass(), body.radius(), body.getPosition(), body.getVelocity(), body.color, null, temperature);
    }
}

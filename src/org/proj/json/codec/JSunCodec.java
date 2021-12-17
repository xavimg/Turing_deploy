package org.proj.json.codec;

import org.proj.game.body.SpaceBody;
import org.proj.game.body.Sun;
import org.proj.json.JSONCodec;
import org.proj.json.JSONObject;

public class JSunCodec implements JSONCodec<Sun> {
    final public static JSunCodec INSTANCE = new JSunCodec();
    private JSunCodec () {}

    @Override
    public JSONObject encode (Sun value) {
        return JSpaceBodyCodec.INSTANCE.encode(value).put("temperature", value.temperature);
    }

    @Override
    public Sun decode (JSONObject json) {
        SpaceBody body = JSpaceBodyCodec.INSTANCE.decode(json);
        double temperature = json.getDouble("temperature");

        return new Sun(body.restMass(), body.radius(), body.getPosition(), body.getVelocity(), body.color, null, temperature);
    }

    @Override
    public Class<Sun> getTransformClass() {
        return Sun.class;
    }
}

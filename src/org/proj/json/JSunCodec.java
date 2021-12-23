package org.proj.json;

import org.proj.game.body.SpaceBody;
import org.proj.game.body.Sun;
import org.sjr.JSONObj;
import org.sjr.codec.JSONCodec;

public class JSunCodec implements JSONCodec<Sun> {
    final public static JSunCodec INSTANCE = new JSunCodec();
    private JSunCodec () {}

    @Override
    public JSONObj encode (Sun value) {
        var encode = JSpaceBodyCodec.INSTANCE.encode(value);
        encode.put("temperature", value.temperature);

        return encode;
    }

    @Override
    public Sun decode (JSONObj json) {
        SpaceBody body = JSpaceBodyCodec.INSTANCE.decode(json);
        double temperature = json.getDouble("temperature").get();

        return new Sun(body.restMass(), body.radius(), body.getPosition(), body.getVelocity(), body.color, null, temperature);
    }

    @Override
    public Class<Sun> getTargetClass() {
        return Sun.class;
    }
}

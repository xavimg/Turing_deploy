package org.proj.json.codec;

import org.json.simple.JSONObject;
import org.proj.game.body.Planet;
import org.proj.game.body.SpaceBody;
import org.proj.json.JSONCodec;
import org.sjr.JSONObjectWrapper;

public class JPlanetCodec implements JSONCodec<Planet> {
    final public static JPlanetCodec INSTANCE = new JPlanetCodec();
    private JPlanetCodec() {}

    @Override
    public JSONObject encode (Planet value) {
        return JSpaceBodyCodec.INSTANCE.encode(value);
    }

    @Override
    public Planet decode (JSONObjectWrapper json) {
        SpaceBody body = JSpaceBodyCodec.INSTANCE.decode(json);
        return new Planet(body.restMass(), body.radius(), body.getPosition(), body.getVelocity(), body.color, null);
    }
}

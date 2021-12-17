package org.proj.json.codec;

import org.proj.game.body.Planet;
import org.proj.game.body.SpaceBody;
import org.proj.json.JSONCodec;
import org.proj.json.JSONObject;

public class JPlanetCodec implements JSONCodec<Planet> {
    final public static JPlanetCodec INSTANCE = new JPlanetCodec();
    private JPlanetCodec() {}

    @Override
    public JSONObject encode (Planet value) {
        return JSpaceBodyCodec.INSTANCE.encode(value);
    }

    @Override
    public Planet decode (JSONObject json) {
        SpaceBody body = JSpaceBodyCodec.INSTANCE.decode(json);
        return new Planet(body.restMass(), body.radius(), body.getPosition(), body.getVelocity(), body.color, null);
    }

    @Override
    public Class<Planet> getTransformClass() {
        return Planet.class;
    }
}

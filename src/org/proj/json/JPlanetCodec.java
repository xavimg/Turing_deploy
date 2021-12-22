package org.proj.json;

import org.proj.game.body.Planet;
import org.proj.game.body.SpaceBody;
import org.sjr.JSONObj;
import org.sjr.codec.JSONCodec;

public class JPlanetCodec implements JSONCodec<Planet> {
    final public static JPlanetCodec INSTANCE = new JPlanetCodec();
    private JPlanetCodec() {}

    @Override
    public JSONObj encode (Planet value) {
        return JSpaceBodyCodec.INSTANCE.encode(value);
    }

    @Override
    public Planet decode (JSONObj json) {
        SpaceBody body = JSpaceBodyCodec.INSTANCE.decode(json);
        return new Planet(body.restMass(), body.radius(), body.getPosition(), body.getVelocity(), body.color, null);
    }

    @Override
    public Class<Planet> getTargetClass() {
        return Planet.class;
    }
}

package org.proj.json;

import org.proj.game.body.SpaceBody;
import org.proj.math.vector.Vec2;
import org.sjr.JSONObj;
import org.sjr.codec.JSONCodec;

import java.awt.*;

public class JSpaceBodyCodec implements JSONCodec<SpaceBody>{
    final public static JSpaceBodyCodec INSTANCE = new JSpaceBodyCodec();
    private JSpaceBodyCodec () {}

    @Override
    public SpaceBody decode (JSONObj json) {
        double restMass = json.getDouble("rest_mass").getAsDouble();
        double radius = json.getDouble("radius").getAsDouble();
        Vec2 position = json.getDecodable("position", JTwoVectorCodec.INSTANCE).get();
        Vec2 velocity = json.getDecodable("velocity", JTwoVectorCodec.INSTANCE).get();
        Color color = new Color(json.getInt("position").getAsInt(), true);

        return new SpaceBody(restMass, radius, position, velocity, null, color, null);
    }

    @Override
    public JSONObj encode (SpaceBody value) {
        JSONObj json = new JSONObj();
        json.put("rest_mass", value.restMass());
        json.put("radius", value.radius());
        json.put("position", JTwoVectorCodec.INSTANCE, value.getPosition());
        json.put("velocity", JTwoVectorCodec.INSTANCE, value.getVelocity());

        if (value.color != null) json.put("color", value.color.getRGB());
        return json;
    }

    @Override
    public Class<SpaceBody> getTargetClass() {
        return SpaceBody.class;
    }
}

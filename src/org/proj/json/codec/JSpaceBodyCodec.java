package org.proj.json.codec;

import org.proj.game.body.SpaceBody;
import org.proj.json.JSONObject;
import org.proj.json.JSONCodec;
import org.proj.math.vector.Vec2;

import java.awt.*;

public class JSpaceBodyCodec implements JSONCodec<SpaceBody> {
    final public static JSpaceBodyCodec INSTANCE = new JSpaceBodyCodec();
    private JSpaceBodyCodec () {}

    @Override
    public SpaceBody decode (JSONObject json) {
        double restMass = json.getDouble("rest_mass");
        double radius = json.getDouble("radius");
        Vec2 position = json.get("position", JTwoVectorCodec.INSTANCE);
        Vec2 velocity = json.get("velocity", JTwoVectorCodec.INSTANCE);
        Color color = json.isNull("color") ? null : new Color(json.getInt("color"), true);

        return new SpaceBody(restMass, radius, position, velocity, null, color, null);
    }

    @Override
    public JSONObject encode (SpaceBody value) {
        JSONObject json = new JSONObject();
        json.put("rest_mass", value.restMass());
        json.put("radius", value.radius());
        json.put("position", JTwoVectorCodec.INSTANCE, value.getPosition());
        json.put("velocity", JTwoVectorCodec.INSTANCE, value.getVelocity());

        if (value.color != null) json.put("color", value.color.getRGB());
        return json;
    }

    @Override
    public Class<SpaceBody> getTransformClass() {
        return SpaceBody.class;
    }
}

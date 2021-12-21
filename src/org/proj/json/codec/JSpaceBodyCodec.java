package org.proj.json.codec;

import org.json.simple.JSONObject;
import org.proj.game.body.SpaceBody;
import org.proj.json.JSONCodec;
import org.proj.math.vector.Vec2;
import org.sjr.JSONObjectWrapper;

import java.awt.*;

public class JSpaceBodyCodec implements JSONCodec<SpaceBody> {
    final public static JSpaceBodyCodec INSTANCE = new JSpaceBodyCodec();
    private JSpaceBodyCodec () {}

    @Override
    public SpaceBody decode (JSONObjectWrapper json) {
        double restMass = json.getDouble("rest_mass").getAsDouble();
        double radius = json.getDouble("radius").getAsDouble();
        Vec2 position = JTwoVectorCodec.INSTANCE.decode(json.getObject("position").get());
        Vec2 velocity = JTwoVectorCodec.INSTANCE.decode(json.getObject("velocity").get());
        Color color = new Color(json.getInt("position").getAsInt(), true);

        return new SpaceBody(restMass, radius, position, velocity, null, color, null);
    }

    @Override
    public JSONObject encode (SpaceBody value) {
        JSONObject json = new JSONObject();
        json.put("rest_mass", value.restMass());
        json.put("radius", value.radius());
        json.put("position", JTwoVectorCodec.INSTANCE.encode(value.getPosition()));
        json.put("velocity", JTwoVectorCodec.INSTANCE.encode(value.getVelocity()));

        if (value.color != null) json.put("color", value.color.getRGB());
        return json;
    }
}

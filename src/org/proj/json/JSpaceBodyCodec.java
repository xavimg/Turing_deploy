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
        json.setSupplier(JTwoVectorCodec.INSTANCE);

        double restMass = json.getDouble("rest_mass").get();
        double radius = json.getDouble("radius").get();
        Vec2 position = json.getAs("position", Vec2.class).get();
        Vec2 velocity = json.getAs("velocity", Vec2.class).get();
        Color color = new Color(json.getInt("position").get(), true);

        return new SpaceBody(restMass, radius, position, velocity, null, color, null);
    }

    @Override
    public JSONObj encode (SpaceBody value) {
        JSONObj json = new JSONObj();
        json.setSupplier(JTwoVectorCodec.INSTANCE);

        json.put("rest_mass", value.restMass());
        json.put("radius", value.radius());

        var pos = json.put("position", value.getPosition());
        var vel = json.put("velocity", value.getVelocity());

        if (pos.isPresent()) {
            throw new RuntimeException(pos.get());
        } else if (vel.isPresent()) {
            throw new RuntimeException(vel.get());
        }

        if (value.color != null) json.put("color", value.color.getRGB());
        return json;
    }

    @Override
    public Class<SpaceBody> getTargetClass() {
        return SpaceBody.class;
    }
}

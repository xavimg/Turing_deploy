package org.proj.json;

import org.proj.math.vector.Vec2;
import org.sjr.JSONObj;
import org.sjr.codec.JSONCodec;

public class JTwoVectorCodec implements JSONCodec<Vec2> {
    final public static JTwoVectorCodec INSTANCE = new JTwoVectorCodec();
    private JTwoVectorCodec () {}

    @Override
    public JSONObj encode (Vec2 value) {
        JSONObj object = new JSONObj();
        object.put("x", value.x);
        object.put("y", value.y);

        return object;
    }

    @Override
    public Vec2 decode (JSONObj json) {
        return new Vec2(json.getDouble("x").get(), json.getDouble("y").get());
    }

    @Override
    public Class<Vec2> getTargetClass() {
        return Vec2.class;
    }
}

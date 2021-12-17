package org.proj.json.codec;

import org.proj.json.JSONCodec;
import org.proj.json.JSONObject;
import org.proj.math.vector.Vec2;

public class JTwoVectorCodec implements JSONCodec<Vec2> {
    final public static JTwoVectorCodec INSTANCE = new JTwoVectorCodec();
    private JTwoVectorCodec () {}

    @Override
    public JSONObject encode (Vec2 value) {
        JSONObject object = new JSONObject();
        object.put("x", value.x);
        object.put("y", value.y);

        return object;
    }

    @Override
    public Vec2 decode (JSONObject json) {
        return new Vec2(json.getDouble("x"), json.getDouble("y"));
    }

    @Override
    public Class<Vec2> getTransformClass() {
        return Vec2.class;
    }
}

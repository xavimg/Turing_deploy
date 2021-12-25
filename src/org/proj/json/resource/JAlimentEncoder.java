package org.proj.json.resource;

import org.proj.game.Aliment;
import org.sjr.JSONObj;
import org.sjr.codec.JSONEncoder;

public class JAlimentEncoder implements JSONEncoder<Aliment> {
    final public static JAlimentEncoder INSTANCE = new JAlimentEncoder();
    private JAlimentEncoder () {}

    @Override
    public JSONObj encode (Aliment aliment) {
        JSONObj res = JResourceEncoder.INSTANCE.encode(aliment);
        res.put("hunger", aliment.getHunger());
        res.put("thirst", aliment.getThirst());
        res.put("health", aliment.getHealth());

        return res;
    }
}

using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using WebSocketUtils;

public class ExternalManager : MonoBehaviour {
    long lastUpdate = 0;

    public void MoveTo (PlayerUpdate update) {
        if (update.at <= lastUpdate) return;
        gameObject.transform.rotation = GetRotation(update.dir);
        gameObject.transform.position = new Vector3(update.position.position.x, update.position.position.y, -4);
        lastUpdate = update.at;
    }

    Quaternion GetRotation(float deg) {
        var rotation = Quaternion.AngleAxis(deg, Vector3.forward);
        return rotation * Quaternion.AngleAxis(-deg, Vector3.right);
    }
}

using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using WebSocketUtils;

public class ExternalManager : MonoBehaviour {
    private long lastTimestamp = -1;
    private volatile float updateTimeout;

    private PlayerUpdate? nextUpdate = null;
    private SortedSet<PlayerUpdate> buffer = new SortedSet<PlayerUpdate>();

    private void FixedUpdate () {
        if (nextUpdate != null) {
            updateTimeout = Mathf.Max(0.0f, updateTimeout - Time.fixedDeltaTime);
            if (updateTimeout > 0) return;

            gameObject.transform.rotation = GetRotation(nextUpdate.dir);
            gameObject.transform.position = new Vector3(nextUpdate.position.x, nextUpdate.position.y, -4);

            lastTimestamp = nextUpdate.at;
            nextUpdate = null;
        }

        if (buffer.Count == 0) return;
        PlayerUpdate update;
        lock (buffer) {
            update = buffer.Min;
            buffer.Remove(update);
        }

        nextUpdate = update;
        if (lastTimestamp == -1) updateTimeout = 0;
        else updateTimeout = Mathf.Min(0.3f, (update.at - lastTimestamp) / 1000.0f);
    }

    public void MoveTo (PlayerUpdate update) {
        lock (buffer) buffer.Add(update);
    }

    public void DestroySelf () {
        Destroy(gameObject);
    }

    Quaternion GetRotation(float deg) {
        var rotation = Quaternion.AngleAxis(deg, Vector3.forward);
        return rotation * Quaternion.AngleAxis(-deg, Vector3.right);
    }
}

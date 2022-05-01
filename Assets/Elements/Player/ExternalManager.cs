using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ExternalManager : MonoBehaviour {
    public void MoveTo (Vector2 dst) {
        gameObject.transform.position = new Vector3(dst.x, dst.y, -4);
    }
}

using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Lifetime : MonoBehaviour {
    public float remaining;

    // Update is called once per frame
    void Update() {
        remaining -= Time.deltaTime;
        if (remaining <= 0) {
            Destroy(gameObject);
        }
    }
}

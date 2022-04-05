using System.Collections;
using System.Collections.Generic;
using System;
using UnityEngine;

public class Movement : MonoBehaviour {
    public float speed;
    private Rigidbody2D rb;

    void Start() {
        rb = gameObject.GetComponent<Rigidbody2D>();
    }

    void Update() {
        /*
        // Right click
        if (PrimaryInput) {
            rb.rotation = Mathf.Rad2Deg * Mathf.Atan2(dir.Value.y, dir.Value.x) - 90f;
            rb.velocity = dir.Value * speed;
        } else {
            rb.velocity = Vector2.zero;
        }

        // Left click
        if (SecondaryInput) {
            Shoot(dir.Value);
        }*/
    }

    /* --- Methods --- */
    void Shoot (Vector2 dir) {
        var ray = new Ray2D(rb.position, dir);
    }

    /* --- Utils --- */
    Vector2? getPrimaryClick() {
        if (Input.GetMouseButton(0)) {
            var halfScreen = new Vector2(Screen.width, Screen.height) / 2;
            var position = new Vector2(Input.mousePosition.x, Input.mousePosition.y) - halfScreen;
            return position.normalized;
        }

        if (Input.touchCount > 0) {

        }

        return null;
    }
}

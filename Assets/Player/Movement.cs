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
        var dir = new Lazy<Vector2>(GetDirection);

        // Right click
        if (PrimaryInput) {
            Move(dir.Value);
        }

        // Left click
        if (SecondaryInput) {
            Shoot(dir.Value);
        }
    }

    /* --- Methods --- */
    void Move (Vector2 dir) {
        rb.AddForce(Time.deltaTime * dir * speed);
    }

    void Shoot (Vector2 dir) {
        var ray = new Ray2D(rb.position, dir);
    }

    /* --- Utils --- */
    bool PrimaryInput {
        get {
            return Input.GetMouseButton(0) || Input.touchCount > 0;
        }
    }

    bool SecondaryInput {
        get {
            return Input.GetMouseButton(1);
        }
    }

    Vector2 GetDirection () {
        var halfScreen = new Vector2(Screen.width, Screen.height) / 2;
        var position = new Vector2(Input.mousePosition.x, Input.mousePosition.y) - halfScreen;
        return position.normalized;
    }
}

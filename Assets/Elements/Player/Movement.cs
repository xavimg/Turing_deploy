using System.Collections;
using System.Collections.Generic;
using System;
using UnityEngine;

public class Movement : MonoBehaviour {
    public float speed;
    public float bulletForce;
    public GameObject bullet;

    private RemoteManager remote;
    private Rigidbody2D rb;

    void Start() {
        RenderSettings.ambientLight = Color.white;
        RenderSettings.ambientIntensity = 100f;
        rb = GetComponent<Rigidbody2D>();
        remote = GetComponent<RemoteManager>();
    }

    void Update() {
        var dir = GetDirection();

        // Movement
        if (Input.GetMouseButton(0)) {
            gameObject.transform.rotation = GetRotation(Mathf.Rad2Deg * Mathf.Atan2(dir.y, dir.x));
            rb.velocity = dir * speed;
        } else {
            rb.velocity = Vector2.zero;
        }

        // Shooting
        if (Input.GetKeyDown(KeyCode.Space)) {
            var bullet = Instantiate(this.bullet, gameObject.transform.position, Quaternion.identity);
            var rbBullet = bullet.GetComponent<Rigidbody2D>();
            rbBullet.AddForce(dir * bulletForce);
        }
    }

    void FixedUpdate() {
        if (remote != null && gameObject.transform.hasChanged)
            remote.UpdateSelf(new Vector2(gameObject.transform.position.x, gameObject.transform.position.y));
    }

    /* --- Utils --- */
    Vector2 GetDirection() {
        var halfScreen = new Vector2(Screen.width, Screen.height) / 2;
        var position = new Vector2(Input.mousePosition.x, Input.mousePosition.y) - halfScreen;
        return position.normalized;
    }

    Quaternion GetRotation (float deg) {
        var rotation = Quaternion.AngleAxis(deg, Vector3.forward);
        return rotation * Quaternion.AngleAxis(-deg, Vector3.right);
    }
}

using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CameraScript : MonoBehaviour {
    public GameObject player;
    public GameObject background1;
    private GameObject background2;

    private bool usePrimary = true;
    private float maxDist;

    // Start is called before the first frame update
    void Start() {
        var sprite = background1.GetComponent<SpriteRenderer>();
        background2 = Instantiate(background1);
        background2.AddComponent<SpriteRenderer>();
        background2.GetComponent<SpriteRenderer>().sprite = sprite.sprite;
        background2.SetActive(false);
        maxDist = sprite.size.magnitude / 4;
    }

    // Update is called once per frame
    void Update() {
        var lastPosition = gameObject.transform.position;
        gameObject.transform.position = new Vector3(player.transform.position.x, player.transform.position.y, -10);

        var deltaPosition = gameObject.transform.position - lastPosition;
        background.transform.position += deltaPosition / 2;

        var dist = (background.transform.position - player.transform.position).magnitude;
        print("" + dist + ", " + maxDist);

        if (dist >= maxDist) {
            usePrimary = !usePrimary;
            background.SetActive(true);
            background2.SetActive(false);
            background.transform.position = gameObject.transform.position;
        }
    }

    GameObject background {
        get { return usePrimary ? background1 : background2; }
    }
}

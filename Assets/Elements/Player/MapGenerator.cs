using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using WebSocketUtils;

public class MapGenerator : MonoBehaviour {
    static UnityEngine.Color TRANSPARENT = new UnityEngine.Color(0, 0, 0, 0);

    public RawImage image;
    public Star star;
    public float starRadius;
    public Planet[] planets;
    public UnityEngine.Color playerColor;

    private Vector2Int? lastPosition;
    private Texture2D texture;
    private Vector2 offset;
    private Vector2 min;
    private Vector2 max;

    // Start is called before the first frame update
    void Start() {
        texture = new Texture2D(1920, 1080, TextureFormat.RGBA32, false);
        image.texture = texture;

        // Set all pixels to transparent
        for (int x = 0; x < texture.width; x++) {
            for (int y = 0; y < texture.height; y++) {
                texture.SetPixel(x, y, TRANSPARENT);
            }
        }

        min = new Vector2(float.PositiveInfinity, float.PositiveInfinity);
        max = new Vector2(float.NegativeInfinity, float.NegativeInfinity);

        // Find maximums and minimums
        foreach (Planet planet in planets) {
            if (planet.position.x < min.x) min.x = planet.position.x;
            if (planet.position.y < min.y) min.y = planet.position.y;
            if (planet.position.x > max.x) max.x = planet.position.x;
            if (planet.position.y > max.y) max.y = planet.position.y;
        }

        offset = Size / 2;
        float radiusScale = 1000 / starRadius;

        // Draw Star
        FillCircle(Size / 2, 50, star.color.Unity);

        // Draw planets
        foreach (Planet planet in planets) {
            print(planet.radius * radiusScale + ", " + radiusScale);
            FillCircle(TexturePosition(planet.position), (int) (planet.radius * radiusScale), planet.color.Unity);
        }

        texture.Apply();
    }

    public void UpdatePlayer (double x, double y) {
        if (lastPosition != null) FillSquare(lastPosition.Value, 25, TRANSPARENT, UnityEngine.Color.white);
        var pos = TexturePosition(x, y);
        FillSquare(pos, 25, playerColor, UnityEngine.Color.white);

        lastPosition = pos;
        texture.Apply();
    }

    Vector2Int TexturePosition(Vector2 world) {
        var delta = (world - min) * FloatSize / (max - min);
        return new Vector2Int((int) delta.x, (int) delta.y);
    }

    Vector2Int TexturePosition(double x, double y) {
        double deltaX = (x - min.x) * texture.width / (max.x - min.x);
        double deltaY = (y - min.y) * texture.height / (max.y - min.y);
        return new Vector2Int((int) deltaX, (int) deltaY);
    }

    Vector2Int Size {
        get { return new Vector2Int(texture.width, texture.height); }
    }

    Vector2 FloatSize {
        get { return new Vector2(texture.width, texture.height); }
    }

    void FillCircle (Vector2Int center, int radius, UnityEngine.Color color) {
        for (int x = Math.Max(0, center.x - radius); x < Math.Min(texture.width, center.x + radius); x++) {
            for (int y = Math.Max(0, center.y - radius); y < Math.Min(texture.height, center.y + radius); y++) {
                var dist = Vector2Int.Distance(center, new Vector2Int(x, y));
                if (dist <= radius) texture.SetPixel(x, y, color);
            }
        }
    }

    void FillSquare (Vector2Int center, int size, UnityEngine.Color color, UnityEngine.Color border) {
        // Filler
        for (int x = Math.Max(0, center.x - size); x < Math.Min(texture.width, center.x + size); x++) {
            // Border
            texture.SetPixel(x, Math.Max(0, center.y - size), border);
            texture.SetPixel(x, Math.Min(texture.height, center.y + size), border);

            // Fill
            for (int y = Math.Max(0, center.y - size) + 1; y < Math.Min(texture.height, center.y + size) - 1; y++) {
                texture.SetPixel(x, y, color);
            }
        }
    }
}

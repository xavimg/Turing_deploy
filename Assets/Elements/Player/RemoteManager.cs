using System;
using System.Collections.Generic;
using System.Text;
using NativeWebSocket;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;
using WebSocketUtils;

public class RemoteManager : MonoBehaviour {
    public GameObject external;
    public RawImage mapImage;

    private MapGenerator? map;
    private WebSocket ws;
    private Dictionary<string, ExternalManager> remotes = new Dictionary<string, ExternalManager>();

    // Start is called before the first frame update
    async void Start() {
        ws = new WebSocket(GameSession.ALEX_URL + "/player/conn", new Dictionary<string, string>() {
            { "Authorization", "Bearer " + GameSession.token }
        });

        ws.OnOpen += () => {
            Debug.Log("Connection open!");
        };

        ws.OnError += (e) => {
            Debug.LogError("Error! " + e);
        };

        ws.OnClose += (e) => {
            Debug.Log("Connection closed! (" + e + ")");
            Application.Quit(0);
        };

        ws.OnMessage += (bytes) => {
            // getting the message as a string
            var message = Encoding.UTF8.GetString(bytes);
            var id = JsonUtility.FromJson<WebSocketId>(message).id;

            switch (id) {
                case 0x10: // Player update
                    UpdatePlayer(JsonUtility.FromJson<WebSocketBody<PlayerUpdate>>(message).body);
                    break;

                case 0x11: // New player
                    AddNewPlayer(JsonUtility.FromJson<WebSocketBody<NewPlayer>>(message).body);
                    break;

                case 0x12: // Current status
                    GetCurrentStatus(JsonUtility.FromJson<WebSocketBody<CurrentStatus>>(message).body);
                    break;

                case 0x13: // Player exit
                    DestroyPlayer(JsonUtility.FromJson<WebSocketBody<PlayerExit>>(message).body);
                    break;
            };
        };

        await ws.Connect();
    }

    // Update is called once per frame
    void Update() {
        if (ws.State == WebSocketState.Closed) {
            SceneManager.LoadScene(0);
            return;
        }

        #if !UNITY_WEBGL || UNITY_EDITOR
            ws.DispatchMessageQueue();
        #endif
    }

    void GetCurrentStatus (CurrentStatus body) {
        if (map != null) return;
        gameObject.transform.position = new Vector3(body.position.x, body.position.y, -4);

        foreach (NewPlayer player in body.players) {
            AddNewPlayer(player);
        }

        // Instantiate planets
        float massRadiusRatio = 0f;
        foreach (Planet planet in body.system.planets) {
            var sphere = GameObject.CreatePrimitive(PrimitiveType.Sphere);
            var radius = planet.radius * Converter.AuToMeters;
            var position = planet.position * Converter.AuToMeters;

            sphere.GetComponent<Renderer>().material.color = planet.color.Unity;
            sphere.transform.position = new Vector3(position.x, position.y, -4);
            sphere.transform.localScale = new Vector3(radius, radius, radius);
            massRadiusRatio += radius / planet.mass;
        }

        // Instantiate sun
        var sun = GameObject.CreatePrimitive(PrimitiveType.Sphere);
        sun.GetComponent<Renderer>().material.color = body.system.star.color.Unity;
        sun.transform.position = new Vector3(0, 0, -4);

        var sunRadius = body.system.star.mass * massRadiusRatio / body.system.planets.Length;
        sun.transform.localScale = new Vector3(sunRadius, sunRadius, sunRadius);

        // Add light to sun
        sun.AddComponent<Light>();
        var light = sun.GetComponent<Light>();
        light.type = LightType.Point;
        light.color = body.system.star.color.Unity;
        light.intensity = 5f;
        light.range = body.system.star.temperature;

        // Generate map
        gameObject.AddComponent<MapGenerator>();
        map = gameObject.GetComponent<MapGenerator>();
        map.image = mapImage;
        map.star = body.system.star;
        map.planets = body.system.planets;
        map.playerColor = body.color.Unity;
        map.starRadius = Converter.MetersToAu * sunRadius;
    }

    void AddNewPlayer (NewPlayer body) {
        var external = Instantiate(this.external, new Vector3(body.location.position.x, body.location.position.y, -4), Quaternion.identity);
        external.AddComponent<ExternalManager>();
        var component = external.GetComponent<ExternalManager>();

        lock (remotes) {
            if (!remotes.TryAdd(body.id, component)) {
                remotes[body.id] = component;
            }
        }
    }

    void UpdatePlayer (PlayerUpdate body) {
        lock (remotes) {
            ExternalManager player;
            if (!remotes.TryGetValue(body.player, out player)) return;
            lock (player) player.MoveTo(body);
        }
    }

    void DestroyPlayer (PlayerExit body) {
        ExternalManager player;
        lock (remotes) {
            if (!remotes.TryGetValue(body.player, out player)) return;
            remotes.Remove(body.player);
        }

        player.DestroySelf();
    }

    public async void UpdateSelf (float dir, Vector2 position) {
        if (ws.State == WebSocketState.Open) {
            double x = position.x * Converter.MetersToAuDouble;
            double y = position.y * Converter.MetersToAuDouble;

            if (map != null) map.UpdatePlayer(x, y);
            var body = new WebSocketBody<SendUpdate>(0x00, new SendUpdate(dir, x, y));
            await ws.SendText(JsonUtility.ToJson(body));
        }
    }

    private async void OnApplicationQuit() {
        if (ws != null) await ws.Close();
    }
}

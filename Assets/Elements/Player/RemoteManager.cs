using System;
using System.Collections;
using System.Collections.Generic;
using System.Text;
using System.Threading;
using NativeWebSocket;
using UnityEngine;
using UnityEngine.SceneManagement;
using WebSocketUtils;

public class RemoteManager : MonoBehaviour {
    public GameObject external;
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
        gameObject.transform.position = new Vector3(body.position.x, body.position.y, -4);
        foreach (NewPlayer player in body.players) {
            AddNewPlayer(player);
        }
    }

    void AddNewPlayer (NewPlayer body) {
        var external = Instantiate(this.external, new Vector3(body.location.position.x, body.location.position.y, -4), Quaternion.identity); ;
        external.AddComponent<ExternalManager>();
        var component = external.GetComponent<ExternalManager>();

        // TODO lock
        if (!remotes.TryAdd(body.id, component)) {
            remotes[body.id] = component;
        }
    }

    void UpdatePlayer (PlayerUpdate body) {
        ExternalManager player = remotes[body.player];
        lock (player) {
            player.MoveTo(body);
        }
    }

    public async void UpdateSelf (float dir, Vector2 position) {
        if (ws.State == WebSocketState.Open) {
            var body = new WebSocketBody<SendUpdate>(0x00, new SendUpdate(dir, position));
            await ws.SendText(JsonUtility.ToJson(body));
        }
    }

    private async void OnApplicationQuit() {
        if (ws != null) await ws.Close();
    }
}

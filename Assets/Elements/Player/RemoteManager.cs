using System;
using System.Collections;
using System.Collections.Generic;
using NativeWebSocket;
using UnityEngine;

public class RemoteManager : MonoBehaviour {
    private WebSocket ws;

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
            Debug.Log("OnMessage!");
            Debug.Log(bytes);

            // getting the message as a string
            // var message = System.Text.Encoding.UTF8.GetString(bytes);
            // Debug.Log("OnMessage! " + message);
        };

        await ws.Connect();
    }

    // Update is called once per frame
    void Update() {
        #if !UNITY_WEBGL || UNITY_EDITOR
            ws.DispatchMessageQueue();
        #endif
    }

    private async void OnApplicationQuit() {
        if (ws != null)
            await ws.Close();
    }
}

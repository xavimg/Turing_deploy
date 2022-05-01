using System;
using System.Runtime.Serialization;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace ServerUtils {
    [Serializable]
    public class ServerResponse<T> {
        public bool status;
        public string message;
        public string[]? errors;
        public T? data;
    }

    [Serializable]
    public class ServerToken {
        public bool status;
        public string message;
        public string[] errors;
        public string token;
    }
}

namespace WebSocketUtils {
    [Serializable]
    public class WebSocketId {
        public byte id;
    }

    [Serializable]
    public class WebSocketBody<T> {
        public byte id;
        public T body;

        public WebSocketBody (byte id, T body) {
            this.id = id;
            this.body = body;
        }
    }

    [Serializable]
    public class NewPlayer {
        public string id;
        public string name;
        public PlayerLocation location;
        public uint color;
    }

    [Serializable]
    public class SendUpdate {
        public Vector2 position;

        public SendUpdate (Vector2 position) {
            this.position = position;
        }
    }

    [Serializable]
    public class PlayerUpdate {
        public string player;
        public Vector2 position;
    }

    [Serializable]
    public class PlayerLocation {
        public string system;
        public Vector2 position;

        public PlayerLocation (string system, Vector2 position) {
            this.system = system;
            this.position = position;
        }
    }
}
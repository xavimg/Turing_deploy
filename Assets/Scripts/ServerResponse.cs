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
        public float dir; // degrees
        public long at; // milliseconds
        public Vector2 position;

        public SendUpdate (float dir, Vector2 position) {
            this.dir = dir;
            this.position = position;

            var now = (DateTimeOffset)DateTime.Now;
            at = now.ToUnixTimeMilliseconds();
        }
    }

    [Serializable]
    public class CurrentStatus {
        public string system;
        public Vector2 position;
        public NewPlayer[] players;
    }

    [Serializable]
    public class PlayerUpdate: IComparable<PlayerUpdate> {
        public string player;
        public float dir; // degrees
        public long at; // milliseconds
        public Vector2 position;

        public int CompareTo(PlayerUpdate other) {
            return at.CompareTo(other.at);
        }
    }

    [Serializable]
    public class PlayerExit {
        public string player;
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
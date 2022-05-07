using System;
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
        public Color color;
    }

    [Serializable]
    public class SendUpdate {
        public float dir; // degrees
        public long at; // milliseconds
        public Vector2Double position;

        public SendUpdate (float dir, Vector2 position) {
            this.dir = dir;
            this.position = new Vector2Double(position);

            var now = (DateTimeOffset)DateTime.Now;
            at = now.ToUnixTimeMilliseconds();
        }

        public SendUpdate(float dir, double x, double y) {
            this.dir = dir;
            this.position = new Vector2Double(x, y);

            var now = (DateTimeOffset)DateTime.Now;
            at = now.ToUnixTimeMilliseconds();
        }
    }

    [Serializable]
    public class CurrentStatus {
        public PlanetarySystem system;
        public Color color;
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

    // Others
    [Serializable]
    public class PlanetarySystem {
        public string id;
        public Star star;
        public Planet[] planets;
    }

    [Serializable]
    public class Star {
        public Color color;
        public float temperature; // Kelvin
        public float mass; // Jupiter masses
    }

    [Serializable]
    public class Planet {
        public uint _id;
        public Color color;
        public float mass; // Jupiter masses
        public float radius; // Astronomical units
        public Vector2 position; // Astronomical units
        public Vector2 velocity;
        // TODO resources
    }

    [Serializable]
    public class Vector2Double {
        public double x;
        public double y;

        public Vector2Double (double x, double y) {
            this.x = x;
            this.y = y;
        }

        public Vector2Double (Vector2 vector) {
            this.x = vector.x;
            this.y = vector.y;
        }
    }

    [Serializable]
    public class Color {
        public byte r;
        public byte g;
        public byte b;

        public UnityEngine.Color Unity {
            get { return new UnityEngine.Color(r / 255.0f, g / 255.0f, b / 255.0f); }
        }
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
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
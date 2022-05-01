using System;
using System.Text;
using TMPro;
using ServerUtils;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Networking;
using UnityEngine.SceneManagement;

public class RegisterHandler : MonoBehaviour {
    private Button btn;
    public Alert alert;
    public TMP_InputField humanName;
    public TMP_InputField email;
    public TMP_InputField password;
    private volatile bool done = false;

    private void Start() {
        btn = GetComponent<Button>();
        btn.onClick.AddListener(OnClick);
    }

    private void Update() {
        if (done) SceneManager.LoadScene(0);
    }

    private void OnClick () {
        var body = new RegisterRequest(humanName.text, email.text, password.text);
        var req = new UnityWebRequest(GameSession.XAVI_URL + "/api/auth/register");

        req.method = "POST";
        req.uploadHandler = new UploadHandlerRaw(Encoding.UTF8.GetBytes(JsonUtility.ToJson(body)));
        req.downloadHandler = new DownloadHandlerBuffer();
        req.SetRequestHeader("Content-Type", "application/json");

        req.SendWebRequest();
        while (!req.isDone) { }

        var resp = JsonUtility.FromJson<ServerResponse<RegisterResponse>>(req.downloadHandler.text);

        if (req.result != UnityWebRequest.Result.Success) {
            var error = (resp == null || resp.errors == null || resp.errors.Length == 0) ? "" : resp.errors[0];
            var msg = resp == null ? "" : (resp.message == null ? "" : resp.message);
            alert.ShowAlert("Error: " + msg, error == null ? "" : error);
            req.Dispose();
            return;
        }

        if (!resp.status) {
            var error = (resp.errors == null || resp.errors.Length == 0) ? "" : resp.errors[0];
            var msg = resp == null ? "" : (resp.message == null ? "" : resp.message);
            alert.ShowAlert("Error: " + msg, error == null ? "" : error);
            req.Dispose();
            return;
        }

        req.Dispose();
        done = true;
    }

    [Serializable]
    class RegisterRequest {
        public string name;
        public string email;
        public string password;

        public RegisterRequest (string name, string email, string password) {
            this.name = name;
            this.email = email;
            this.password = password;
        }
    }

    [Serializable]
    class RegisterResponse {
        public ulong id;
        public string name;
        public string email;
    }
}
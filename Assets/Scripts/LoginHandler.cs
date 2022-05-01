using System;
using System.Text;
using TMPro;
using ServerUtils;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Networking;
using UnityEngine.SceneManagement;

public class LoginHandler : MonoBehaviour {
    private Button btn;
    public Alert alert;
    public TMP_InputField email;
    public TMP_InputField password;
    private volatile bool done = false;

    private void Start() {
        btn = GetComponent<Button>();
        btn.onClick.AddListener(OnClick);
    }

    private void Update() {
        if (done) SceneManager.LoadScene(3);
    }

    private void OnClick () {
        var body = new LoginRequest(email.text, password.text);
        var req = new UnityWebRequest(GameSession.XAVI_URL + "/api/auth/login");

        req.method = "POST";
        req.uploadHandler = new UploadHandlerRaw(Encoding.UTF8.GetBytes(JsonUtility.ToJson(body)));
        req.downloadHandler = new DownloadHandlerBuffer();
        req.SetRequestHeader("Content-Type", "application/json");

        req.SendWebRequest();
        while (!req.isDone) {}

        var resp = JsonUtility.FromJson<ServerToken>(req.downloadHandler.text);

        if (req.responseCode != 200) {
            alert.ShowAlert("Error: " + resp?.message ?? "", resp?.errors[0] ?? "");
            req.Dispose();
            return;
        }

        if (!resp.status) {
            alert.ShowAlert("Error: " + resp?.message ?? "", resp?.errors[0] ?? "");
            req.Dispose();
            return;
        }

        req.Dispose();
        GameSession.token = resp.token;
        done = true;
    }

    [Serializable]
    class LoginRequest {
        public string email;
        public string password; // TODO Maybe hash?

        public LoginRequest (string email, string password) {
            this.email = email;
            this.password = password;
        }
    }
}
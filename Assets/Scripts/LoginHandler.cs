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

    private void Start() {
        btn = GetComponent<Button>();
        btn.onClick.AddListener(OnClick);
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
            alert.ShowAlert("Error: " + resp.message, resp.errors.Length > 0 ? resp.errors[0] : "");
            return;
        }

        if (!resp.status) {
            alert.ShowAlert("Error: " + resp.message, resp.errors.Length > 0 ? resp.errors[0] : "");
            return;
        }

        // Load game scene with obtained token
        print(resp.token);
        GameSession.token = resp.token;
        SceneManager.LoadScene(3);
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
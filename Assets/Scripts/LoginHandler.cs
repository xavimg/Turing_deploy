using System;
using System.Text;
using System.Collections;
using TMPro;
using ServerUtils;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Networking;
using UnityEngine.SceneManagement;

public class LoginHandler : MonoBehaviour {
    const string URL = "http://localhost:3000";

    private Button btn;
    public Alert alert;
    public TMP_InputField email;
    public TMP_InputField password;

    private void Start() {
        btn = GetComponent<Button>();
        btn.onClick.AddListener(OnClick);
    }

    private void OnClick () {
        StartCoroutine(ExecLoginRequest());
    }

    private IEnumerator ExecLoginRequest () {
        var body = new LoginRequest(email.text, password.text);
        var req = new UnityWebRequest(URL + "/api/auth/login");

        req.method = "POST";
        req.uploadHandler = new UploadHandlerRaw(Encoding.UTF8.GetBytes(JsonUtility.ToJson(body)));
        req.downloadHandler = new DownloadHandlerBuffer();
        req.SetRequestHeader("Content-Type", "application/json");

        yield return req.SendWebRequest();
        var resp = JsonUtility.FromJson<ServerToken>(req.downloadHandler.text);

        if (req.responseCode != 200) {
            alert.ShowAlert("Error: " + resp.message, resp.errors[0]);
            yield break;
        }

        if (!resp.status) {
            alert.ShowAlert("Error: " + resp.message, resp.errors[0]);
            yield break;
        }

        // Load game scene with obtained token
        GameSession.token = resp.token;
        this.alert = null;
        SceneManager.LoadScene(2);
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
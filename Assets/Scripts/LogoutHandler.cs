using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Networking;
using UnityEngine.SceneManagement;

public class LogoutHandler : MonoBehaviour {
    private Button btn;
    public Alert alert;

    // Start is called before the first frame update
    void Start() {
        btn = GetComponent<Button>();
        btn.onClick.AddListener(OnClick);
    }

    void OnClick () {
        // localhost:8081/api/auth/logout
        var req = new UnityWebRequest(GameSession.XAVI_URL + "/api/auth/logout");

        req.method = "POST";
        req.downloadHandler = new DownloadHandlerBuffer();
        req.SetRequestHeader("Content-Type", "application/json");

        req.SendWebRequest();
        while (!req.isDone) { }

        if (req.responseCode != 200) {
            alert.ShowAlert("Error", "Unknown  error logging out");
            return;
        }

        // Load game scene with obtained token
        GameSession.token = null;
        SceneManager.LoadScene(0);
    }
}

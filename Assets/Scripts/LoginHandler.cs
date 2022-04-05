using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public class LoginHandler : MonoBehaviour {
    private Button btn;
    public TMP_InputField username;
    public TMP_InputField password;

    private void Start() {
        btn = GetComponent<Button>();
        btn.onClick.AddListener(OnClick);
    }

    private void OnClick() {
        var username = this.username.text;
        var password = this.password.text;

        print("Username: " + username + "\n Password: " + password);
    }
}

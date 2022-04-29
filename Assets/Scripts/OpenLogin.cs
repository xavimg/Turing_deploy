using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.SceneManagement;

public class OpenLogin : MonoBehaviour {
    public Button login;

    // Start is called before the first frame update
    void Start() {
        login.onClick.AddListener(OnLogin);
    }

    void OnLogin () {
        SceneManager.LoadScene(1);
    }
}

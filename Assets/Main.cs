using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.SceneManagement;

public class Main : MonoBehaviour {
    public Button login;
    public Button register;
    public Button tutorial;

    void Start () {
        login.onClick.AddListener(() => SceneManager.LoadScene(1));
        register.onClick.AddListener(() => SceneManager.LoadScene(2));
        tutorial.onClick.AddListener(() => SceneManager.LoadScene(4));
    }
}

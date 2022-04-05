using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public class Alert: MonoBehaviour {
    public TMP_Text title;
    public TMP_Text message;
    public Button dismiss;

    private void Awake () {
        dismiss.onClick.AddListener(HideDialog);
    }

    public void HideDialog () {
        gameObject.SetActive(false);
    }

    public void ShowAlert (string title, string message) {
        this.title.SetText(title);
        this.message.SetText(message);
        gameObject.SetActive(true);
    }
}

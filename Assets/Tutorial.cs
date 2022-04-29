using System.Collections;
using System.Collections.Generic;
using System.Threading.Tasks;
using UnityEngine;
using UnityEngine.SceneManagement;

public class Tutorial : MonoBehaviour {
    const int DELAY_MILLIS = 1500;

    public Alert alert;
    private TutorialSection current;

    // Start is called before the first frame update
    void Start() {
        current = TutorialSection.Movement;
        ShowStep();
    }

    // Update is called once per frame
    void Update() {
        if (TaskCompleted) {
            var next = NextSection;
            if (next == null) {
                SceneManager.LoadScene(0);
                return;
            }

            current = next.Value;
            ShowStep();
        }
    }

    async void ShowStep () {
        await Task.Delay(DELAY_MILLIS);
        switch (current) {
            case TutorialSection.Movement:
                alert.ShowAlert("Movement", "Click your mouse on the direction you want to move to");
                break;

            case TutorialSection.Shooting:
                alert.ShowAlert("Shooting", "Press 'Space' to shoot");
                break;
        }

        await Task.Delay(DELAY_MILLIS);
        alert.HideDialog();
    }

    bool TaskCompleted {
        get {
            switch (current) {
                case TutorialSection.Movement:
                    return Input.GetMouseButton(0);

                case TutorialSection.Shooting:
                    return Input.GetKey(KeyCode.Space);
            }

            return false;
        }
    }

    TutorialSection? NextSection {
        get {
            switch (current) {
                case TutorialSection.Movement:
                    return TutorialSection.Shooting;

                case TutorialSection.Shooting:
                    return null;
            }

            return null;
        }
    }
}

enum TutorialSection {
    Movement,
    Shooting
}

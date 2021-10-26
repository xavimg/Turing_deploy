class Player {
    
    /**
     * Constructs player from Object.
     * @param {Object} json
     */
    constructor(json) {
        this.sprite = PIXI.Sprite.from(json.player.texture);
        this.lockSpritePosition = true;
        this.setPosition(json.player.position.x, json.player.position.y);
        this.setRotation(json.player.rotation);
        this.sprite.anchor = {x: 0.5, y: 0.5};

        this.name = json.player.name;
        this.currentSystem = json.player.currentSystem;
        this.currentPlanet = json.player.currentPlanet;
    }

    setRotation(rotation) {
        this.rotation = rotation;
        this.sprite.rotation = rotation;
    }

    setPosition(x, y) {
        this.position = {x: x, y: y};
        if (!this.lockSpritePosition)
            this.sprite.position = this.position;
    }

}
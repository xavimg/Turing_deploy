class Planet {
    
    constructor(json) {
        this.id = json.id;
        this.name = json.name;
        this.radius = json.radius;
        this.angularSpeed = json.angularSpeed;
        this.position = json.position;
        this.color = json.color;
        this.atmosphere = new Atmosphere(json.atmosphere, this.position);
        this.sprite = new PIXI.Container();
        debugger
        this.sprite.addChild(this.createPlanetSprite());
        this.sprite.addChild(this.atmosphere.sprite);
    }

    createPlanetSprite() {
        const gfx = new PIXI.Graphics();
        gfx.beginFill(this.color);
        gfx.drawCircle(this.position.x, this.position.y, this.radius);
        gfx.endFill();
        return gfx;
    }

}
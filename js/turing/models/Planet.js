class Planet {
    
    constructor(json) {
        this.id = json.planet.id;
        this.name = json.planet.name;
        this.radius = json.planet.radius;
        this.angularSpeed = json.planet.angularSpeed;
        this.position = json.planet.position;
        this.color = json.planet.color;
        this.atmosphere = new Atmosphere(json.planet.atmosphere);
        this.sprite = new PIXI.Container();
        this.sprite.addChild(this.createPlanetSprite(radius, color));
        this.sprite.addChild(this.atmosphere);
    }

    createPlanetSprite() {
        const gfx = new PIXI.Graphics();
        gfx.beginFill(this.color);
        gfx.drawCircle(this.position.x, this.position.y, this.radius);
        gfx.endFill();
        return gfx;
    }

}
class Atmosphere {

    constructor(json, position) {
        this.position = position;
        this.sprite = new PIXI.Container();
        this.layers = [];
        for (let i = 0; i < json.layers.length; i++) {
            this.layers.push(this.createLayer(json.layers[i].radius, json.layers[i].color, json.layers[i].alpha));
            this.sprite.addChild(this.layers[i]);
        }
    }

    createLayer(radius, color, alpha) {
        const gfx = new PIXI.Graphics();
        gfx.beginFill(color, alpha);
        gfx.drawCircle(this.position.x, this.position.y, radius);
        gfx.endFill();
        return gfx;
    }

}
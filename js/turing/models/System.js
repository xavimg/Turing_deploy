class System {

    constructor(json) {
        this.id = json.system.id;
        this.size = json.system.size;
        this.planets = [];
        this.layers = [];
        this.sprite = new PIXI.Container();
        for (let i = 0; i < json.system.planets.size; i++) {
            this.planets.push(new Planet(json.system.planets[i]));
            this.sprite.addChild(this.planets[this.planets.length]);
        }
        this.createStarBackground();
    }

    createStarLayer(color, density) {
        const layer = new PIXI.Graphics();
        layer.beginFill(color, 1);
        for (let i = 0; i < Math.pow(2, density); i++) {
            layer.drawRect(Math.random() * this.size.x, Math.random() * this.size.y, Math.random() * 2, Math.random() * 2);
        }
        layer.endFill();
        return layer;
    }

    createStarBackground() {
        this.layers.push(this.createStarLayer(0xFFAAAA, 16)); // red-shifted bg stars
        this.layers.push(this.createStarLayer(0xFFFFFF, 16)); // average bg stars
        this.layers.forEach( layer => this.sprite.addChild(layer));
    }


}
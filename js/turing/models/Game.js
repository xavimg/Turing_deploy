class Game {

    constructor(server, options) {
        this.app = new PIXI.Application(options);
        this.server = server;
        this.player = new Player(JSON.parse(server.getPlayer("self")));
        this.system = new System(JSON.parse(server.getSystem(this.player.currentSystem)));
        this.app.stage.addChild(this.player.sprite);
        this.app.stage.addChild(this.system.sprite);
    }

    moveCamera(x, y) {
        // parallax scroll on system layers 
        // do not move first layer (start from 1)
        const parallaxRate = 0.1;
        for (let i = 1; i < this.system.layers.length; i++) {
            this.system.layers[i].position.x += (x * parallaxRate) * i;
            this.system.layers[i].position.y += (y * parallaxRate) * i;
        }

        // scroll planets & elements
        for (let i = 0; i < this.system.planets.length; i++) {
            this.system.planets[i].sprite.position.x += x;
            this.system.planets[i].sprite.position.y += y;
        }
    }

    movePlayer(x, y) {
        // moves camera
        this.moveCamera(x, y);
    }

}
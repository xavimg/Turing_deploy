class Game {

    constructor(server, options) {
        this.app = new PIXI.Application(options);
        this.server = server;
        this.player = new Player(JSON.parse(server.getPlayer("self")));
        this.system = new System(JSON.parse(server.getSystem(this.player.currentSystem)));
        this.app.stage.addChild(this.system.sprite);
        this.app.stage.addChild(this.player.sprite);
        this.keys = [];
        this.events = [];
        this.player.sprite.x = (this.app.screen.width / 2);
        this.player.sprite.y = (this.app.screen.height / 2);
        this.moveCamera(this.player.sprite.x, this.player.sprite.y);
        // console.log("🌌 Turing started 🌌")
    }

    moveCamera(x, y) {
        // parallax scroll on system layers 
        // do not move first layer (start from 1)
        const parallaxRate = 0.05;
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
        this.moveCamera(x, y);
        this.player.setPosition(x, y);
    }

    setPlayerToScreenCenter() {
        this.player.lockSpritePosition = false;
        this.player.setPosition(this.app.screen.width / 2, this.app.screen.height / 2);
        this.player.lockSpritePosition = true;
    }

}
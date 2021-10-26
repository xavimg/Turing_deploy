class MockServer {
    static getPlanetarySystem(id) {
        let str = 
        `
        {
            "planetarySystem" : {
                "id": 1,
                "size": {
                    "x": 1600,
                    "y": 1600
                },
                "planets" : [
                    {
                        "planet" : {
                            "id": 1,
                            "mass": 100,
                            "radius": 25,
                            "angularSpeed": 0,
                            "position": {
                                "x": 150,
                                "y": 500
                            },
                            "speed": {
                                "x": 0,
                                "y": 0
                            },
                            "color": "0xFF00FF"
                        }
                    },
                    {
                        "planet" : {
                            "id": "",
                            "mass": 500,
                            "radius": 0,
                            "angularSpeed": 50,
                            "position": {
                                "x": 550,
                                "y": 150
                            },
                            "speed": {
                                "x": 0,
                                "y": 0
                            },
                            "color": "0x0000FF"
                        }
                    }
                ]       
            }
        }
        `;
        return str;
    }

    static getPlayer(token) {
        const str =
        `
        {
            "player" : {

                "currentSystemId": 1,
                "currentPlanetId": 1,

                "speed" : {
                    "x": 0,
                    "y": 0
                },

                "position": {
                    "x": 55,
                    "y": 55
                }
            } 
        }
        `;
        return str;
    }
}

/*
const up = 0;
const right = MATH.PI / 2;
const down = MATH.PI;
const left = down + right;*/

class Player {

    constructor(json) {
        this.currentSystemId = json.player.currentSystemId;
        this.currentPlanetId = json.player.currentPlanetId;
        this.speed = json.player.speed;
        this.position = json.player.position;
        this.rotation = 0.0;
        this.sprite = PIXI.Sprite.from("dat/spaceship.png");
        this.sprite.anchor.x = 0.5;
        this.sprite.anchor.y = 0.5;
        this.updateSprite();
    }

    updateSprite() {
        this.sprite.position.x = this.position.x;
        this.sprite.position.y = this.position.y;
        this.sprite.rotation = this.rotation;
    }

    rotate(direction) {
        switch (direction) {
            case "u": {
                this.rotation = 0;
            } break;
            case "r": {
                this.rotation = Math.PI / 2;
            } break;
            case "d": {
                this.rotation = Math.PI;
            } break;
            case "l": {
                this.rotation = Math.PI + (Math.PI / 2);
            } break;
            case "ur":
            case "ru":
            {
                this.rotation = Math.PI / 4;
            } break;
            case "dr":
            case "rd": {
                this.rotation = Math.PI - (Math.PI / 4);
            } break;
            case "dl":
            case "ld": {
                this.rotation = Math.PI + (Math.PI / 4);
            } break;
            case "ul":
            case "lu": {
                this.rotation = Math.PI * 2 - (Math.PI / 4);
            } break;
        }
        this.updateSprite();
    }

    move(direction, step) {
        if (direction.indexOf("u") > -1) {
            this.position.y += -step;
        }
        if (direction.indexOf("d") > -1) {
            this.position.y += +step;
        }
        if (direction.indexOf("r") > -1) {
            this.position.x += +step;
        }
        if (direction.indexOf("l") > -1) {
            this.position.x += -step;
        }
        this.updateSprite();
    }

}

class Planet {
    
    constructor(json) {
        this.id = json.planet.id;
        this.mass = json.planet.mass;
        this.radius = json.planet.radius;
        this.angularSpeed = json.planet.angularSpeed;
        this.position = json.planet.position;
        this.speed = json.planet.speed;
        this.color = json.planet.color;
        this.sprite = this.createSprite();
    }

    createSprite() {
        const graphics = new PIXI.Graphics();
        graphics.lineStyle(1, this.color, 1);
        graphics.beginFill(this.color, 1);
        graphics.drawCircle(this.position.x, this.position.y, this.radius);
        graphics.endFill();
        return graphics;
    }


}

class PlanetarySystem {
    
    constructor(json) {
        this.id = json.planetarySystem.id;
        this.size = {
            x: json.planetarySystem.size.x,
            y: json.planetarySystem.size.y
        }
        this.planets = [];
        for (let i = 0; i < json.planetarySystem.planets.length; i++) {
            this.planets.push(new Planet(json.planetarySystem.planets[i]));
        }
        this.starLayers = [];
        this.container = this.createContainer();
        this.createStarBackground();
    }

    createStarLayer(color, density) {
        const starLayer = new PIXI.Graphics();
        starLayer.beginFill(color, 1);
        for (let i = 0; i < Math.pow(2, density); i++) {
            starLayer.drawRect(Math.random() * this.size.x, Math.random() * this.size.y, Math.random() * 2, Math.random() * 2);
        }
        starLayer.endFill();
        return starLayer;
    }

    createStarBackground() {
        let starLayer;
        
        // Red shifted star bg
        starLayer = this.createStarLayer(0xFFAAAA, 12);
        this.starLayers.push(starLayer);

        // Normal intermedium stars
        starLayer = this.createStarLayer(0xFFFFFF, 12);
        this.starLayers.push(starLayer);

        this.starLayers.forEach( layer => this.container.addChild(layer));
    }

    createContainer() {
        const container = new PIXI.Container();
        this.planets.forEach( element => {
            container.addChild(element.sprite);
        });
        return container;
    }


}
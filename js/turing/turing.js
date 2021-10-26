/*
const up = 0;
const right = MATH.PI / 2;
const down = MATH.PI;
const left = down + right;*/

class Player {

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
        graphics.zIndex = 999;
        graphics.alpha = 0xFFFFFF;
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
        this.starLayers = [];
        this.planets = [];
        this.container = this.createContainer();
        this.createStarBackground();
        for (let i = 0; i < json.planetarySystem.planets.length; i++) {
            this.planets.push(new Planet(json.planetarySystem.planets[i]));
            this.container.addChild(this.planets[this.planets.length-1].sprite);
        }
    }

    createStarLayer(color, density) {
        const starLayer = new PIXI.Graphics();
        starLayer.beginFill(color, 1);
        for (let i = 0; i < Math.pow(2, density); i++) {
            starLayer.drawRect(Math.random() * this.size.x, Math.random() * this.size.y, Math.random() * 2, Math.random() * 2);
        }
        starLayer.endFill();
        starLayer.zIndex = 1;
        return starLayer;
    }

    createStarBackground() {
        let starLayer;
        
        // Red shifted star bg
        starLayer = this.createStarLayer(0xFFAAAA, 16);
        this.starLayers.push(starLayer);

        // Normal intermedium stars
        starLayer = this.createStarLayer(0xFFFFFF, 16);
        this.starLayers.push(starLayer);

        this.starLayers.forEach( layer => this.container.addChild(layer));
    }

    createContainer() {
        const container = new PIXI.Container();
        return container;
    }


}
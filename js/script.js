
const token = "abcd1234";

let player = new Player(
    JSON.parse(
        MockServer.getPlayer(token)
        )
    );

let planetarySystem = new PlanetarySystem(
    JSON.parse(
        MockServer.getPlanetarySystem(player.currentSystemId)
        )
    );

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

function registerPlayerAction(action) {
    // negate up and down directions pressed at the same time
    if (action.direction.indexOf("u") > -1 && action.direction.indexOf("d") > -1) {
        action.direction = action.direction.replace("u", "");
        action.direction = action.direction.replace("d", "");
    }

    // negate right and left directions pressed at the same time
    if (action.direction.indexOf("r") > -1 && action.direction.indexOf("l") > -1) {
        action.direction = action.direction.replace("r", "");
        action.direction = action.direction.replace("l", "");
    }

    const step = 1;
    let finalStepX = 0;
    let finalStepY = 0;

    if (action.direction.indexOf("u") > -1) {
        finalStepY += +step;
    }
    if (action.direction.indexOf("d") > -1) {
        finalStepY += -step;
    }
    if (action.direction.indexOf("r") > -1) {
        finalStepX += -step;
    }
    if (action.direction.indexOf("l") > -1) {
        finalStepX += +step;
    }

    switch (action.type) {
        case "move": {
            player.rotate(action.direction);
            moveCamera(finalStepX, finalStepY);
            player.move(action.direction, step);
        }
    }
}

let keys = [];

function keydown(e) {
    if (!keys.includes(e.keyCode))
        keys.push(e.keyCode);
}

function keyup(e) {
    const index = keys.indexOf(e.keyCode);
    if (index > -1) {
        keys.splice(index, 1);
    }
}

function moveCamera(x, y) {
    /**
     * Parallax effect
     */
    planetarySystem.starLayers[1].position.x += x / 32;
    planetarySystem.starLayers[1].position.y += y / 32;
    planetarySystem.starLayers[0].position.x += x / 64;
    planetarySystem.starLayers[0].position.y += y / 64;
    for (let i = 0; i < planetarySystem.planets.length; i++) {
        planetarySystem.planets[i].sprite.position.x += x;
        planetarySystem.planets[i].sprite.position.y += y;
    }
}

async function keyLookup() {
    while (true) {
        await sleep(1);
        action = {
            type: "move",
            direction: ""
        };
        if (keys.includes(87)) { // W
            action.direction += "u";
        } 
        if (keys.includes(68)) { // D
            action.direction += "r";
        } 
        if (keys.includes(83)) { // S
            action.direction += "d";
        } 
        if (keys.includes(65)) { // A
            action.direction += "l";
        }
        if (action.direction != "") {
            registerPlayerAction(action);
        }                
    }
}

function init() {

    const app = new PIXI.Application(options);
    app.stage.addChild(player.sprite);
    app.stage.addChild(planetarySystem.container);
    document.body.appendChild(app.view);
    player.sprite.position.x = app.view.width / 2;
    player.sprite.position.y = app.view.height / 2;
    document.addEventListener('keydown', keydown);
    document.addEventListener('keyup', keyup);

    keyLookup();
}

const options = {
    width: window.innerWidth,
    height: window.innerHeight,
    backgroundColor: "black"
};

init();
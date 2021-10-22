const options = {
    width: 800,
    height: 800,
    backgroundColor: "black",
    resolution: 1
};

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
    switch (action.type) {
        case "keymovement": {
            player.rotate(action.direction);
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

async function keyLookup() {
    while (true) {
        await sleep(1);
        action = {
            type: "move"
        };
        if (keys.includes(87)) { // W
            action.direction = "u";
        } else 
        if (keys.includes(68)) { // D
            action.direction = "r";
        } else 
        if (keys.includes(83)) { // S
            action.direction = "d";
        } else 
        if (keys.includes(65)) { // A
            action.direction = "l";
        }
        if (action.direction != undefined)
            registerPlayerAction(action);
    }
}

const app = new PIXI.Application(options);
app.stage.addChild(planetarySystem.container);
app.stage.addChild(player.sprite);

document.body.appendChild(app.view);
document.addEventListener('keydown', keydown);
document.addEventListener('keyup', keyup);

keyLookup();
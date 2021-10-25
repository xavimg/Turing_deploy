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

    let step = 1;

    step = step / action.direction.length;

    switch (action.type) {
        case "move": {
            player.rotate(action.direction);
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

const app = new PIXI.Application(options);
app.stage.addChild(planetarySystem.container);
app.stage.addChild(player.sprite);

document.body.appendChild(app.view);
document.addEventListener('keydown', keydown);
document.addEventListener('keyup', keyup);

keyLookup();
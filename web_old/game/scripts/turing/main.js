const options = {
    width: window.innerWidth,
    height: window.innerHeight,
    backgroundColor: "black"
};

let game;
let mockServer;

function init() {
    mockServer = new MockServer("abcd1234", "duke.io");
    game = new Game(mockServer, options);
    document.body.appendChild(game.app.view);
}

function addEventListeners() {
    document.addEventListener('keydown', (e) => {
        if (!game.keys.includes(e.key))
            game.keys.push(e.key);
    });

    document.addEventListener('keyup', (e) => {
        const index = game.keys.indexOf(e.key);
        if (index > -1) {
            game.keys.splice(index, 1);
        }
    });

    window.addEventListener('resize', () => {
        game.app.resizeTo = window;
        game.app.resize();
    });
}

async function startEventLoop() {
    while (true) {
        // busy waiting
        await sleep(1);
        
        let action = null;

        // check keys
        if (game.keys.length > 0) {
            action = {
                type: "move",
                direction: ""
            };
            game.keys.forEach(key => {
                if (key == "w")
                    action.direction += "u";
                if (key == "s")
                    action.direction += "d";
                if (key == "d")
                    action.direction += "r";
                if (key == "a")
                    action.direction += "l";
            });
        }

        // check mouse

        // send action 
        if (action != null) {
            if (action.type == "move")
                if (action.direction != "") {
                    let x = 0, y = 0, r = 0;
                    if (action.direction.indexOf("r") > -1)
                        x = -1;
                    else if (action.direction.indexOf("l") > -1) 
                        x = 1;
                    if (action.direction.indexOf("u") > -1)
                        y = 1;
                    else if (action.direction.indexOf("d") > -1) 
                        y = -1;
                    game.movePlayer(x, y);                    
                    switch (action.direction) {
                        case "u": {
                            r = up;
                        } break;
                        case "r": {
                            r = right;
                        } break;
                        case "d": {
                            r = down;
                        } break;
                        case "l": {
                            r = left;
                        } break;
                        case "ur":
                        case "ru": {
                            r = Math.PI / 4;
                        } break;
                        case "dr":
                        case "rd": {
                            r = Math.PI - (Math.PI / 4);
                        } break;
                        case "dl":
                        case "ld": {
                            r = Math.PI + (Math.PI / 4);
                        } break;
                        case "ul":
                        case "lu": {
                            r = Math.PI * 2 - (Math.PI / 4);
                        } break;
                    }
                    game.player.setRotation(r);
                }
        }
    }
}

try {
    init();
    addEventListeners();
    startEventLoop();
} catch (error) {
    console.log(`🌌 Turing error 🌌: ${error.message}`);
}
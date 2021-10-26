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
        game.setPlayerToScreenCenter();
    });
}

async function startEventLoop() {
    while (true) {
        await sleep(1);
        if (game.keys.length > 0) {
            console.log(`key pressed: ${game.keys}`);
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


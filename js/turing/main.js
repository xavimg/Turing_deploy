const options = {
    width: window.innerWidth,
    height: window.innerHeight,
    backgroundColor: "black"
};
let game;
try {
    let mockServer = new MockServer("abcd1234", "duke.io");
    game = new Game(mockServer, options);
    document.body.appendChild(game.app.view);
} catch (error) {
    console.log(`😵 Turing error 😵: ${error.message}`);
}

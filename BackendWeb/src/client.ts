const connection = new WebSocket("ws://localhost:3000");

function connect () {

connection.onopen = () => {
    
    console.log("Socked has been opened !");
    connection.send("Ping..");
    

};

connection.onmessage = (msg: any) => {

    console.log("Message received !");
    console.log(msg.content.toString());

}
}

window.onload = () => {

    connect();
}

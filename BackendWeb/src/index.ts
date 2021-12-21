import 'reflect-metadata';
import express from 'express';
import morgan from 'morgan';
import cors from 'cors';
import WebSocket from 'ws';
import http from 'http';
import { createConnection } from 'typeorm'
import userRoutes from './routes/userRoutes';

const app = express();

createConnection()

app.use(cors());
app.use(morgan('dev'));
app.use(express.json());

app.use(userRoutes);

app.listen(3000);

// const server = http.createServer(app);

// server.listen(1234);

// const ws = new WebSocket.Server({});

// ws.on('connection', (ws: WebSocket) => {

//     console.log("new user");

//     ws.on('message', (msg: WebSocket.Data) => {


//         if (msg.toString() === 'ok') {
//             console.log("pass");
//         } else {
//             console.log("error");
//         }

//     });

//     ws.send('connected')
// });

const connection = new WebSocket("ws://192.168.233.195:1234");



connection.onopen = () => {
    
    console.log("Socked has been opened !");
    connection.send("Ping..");
    

};

connection.on('message', (msg: any) => {

    console.log("Message received !", msg.toString());
    
})

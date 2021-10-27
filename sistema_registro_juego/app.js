const express = require("express");
const path = require("path");
const mysql = require("mysql");
const dotenv = require("dotenv");
const cookieParser = require("cookie-parser");

dotenv.config({ path: './.env'})

// const controllerPages = require('./controllers/pages_controllers')

const app = express();

const db = mysql.createConnection({
    host: process.env.DATABASE_HOST,
    user: process.env.DATABASE_USER,
    password: process.env.DATABASE_PASSWORD,
    database: process.env.DATABASE
});

const publicDirectory = path.join(__dirname, './public  ');
app.use(express.static(publicDirectory));

// Parse URL-encodes bodies ( html forms , make sure grab data from any form)
app.use(express.urlencoded({ extended: false }))
// Parse JSON bodies ( sent by API clients)
app.use(express.json());
app.use(cookieParser());

console.log(__dirname);
// Template
app.set('view engine', 'hbs');

db.connect( (err) => {
    if (err) {
        console.log("IMPOSIBLE", err);
    } else {
        console.log('MYSQL Connected...')
    }

})

// Define routes to html
app.use('/', require('./routes/pages_routes'));
app.use('/auth', require('./routes/auth'));

// postman routes test
app.use('/users', require('./routes/users'));


app.listen(3000, () => {
    console.log("Server started on port 3000")
})
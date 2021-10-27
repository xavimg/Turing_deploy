const mysql = require("mysql");
const jwt = require("jsonwebtoken");
const bcrypt = require("bcryptjs");
const { promisify } = require("util");
const { restart } = require("nodemon");

const db = mysql.createConnection({
        host: process.env.DATABASE_HOST,
        user: process.env.DATABASE_USER,
        password: process.env.DATABASE_PASSWORD,
        database: process.env.DATABASE
});


exports.getAllUsers = (req, res) => {

    db.query('SELECT * FROM users', (err, result) => {

    resultado = JSON.stringify(result);

    res.status(200).send(resultado);

    })
}

exports.getUserById = async (req, res) => {

    try {

        const { id } = req.params;

        db.query('SELECT name, email FROM users WHERE id = ?', [id], async (err, result) => {

            if ( result.length < 1) {
                res.status(401).send('There is no user with this id');
            } else {
                console.log(result);
                res.status(200).json(result);
            }

        });
    
    }catch (err) {}
}

exports.deleteUserById = async (req, res) => {

    try {

        const { id } = req.params;

        db.query('SELECT name, email FROM users WHERE id = ?', [id], async (err, result) => {

            if ( result.length < 1) {
                res.status(401).send('There is no user with this id');
            } else {
                db.query('DELETE FROM users WHERE id = ?' , [id], async (err, result) => {
                    res.status(200).json(result);
                })
            }

        });
    
    }catch (err) {

    }
}


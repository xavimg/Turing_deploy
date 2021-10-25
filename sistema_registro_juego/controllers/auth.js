const mysql = require("mysql");
const jwt = require("jsonwebtoken");
const bcrypt = require("bcryptjs");

const db = mysql.createConnection({
        host: process.env.DATABASE_HOST,
        user: process.env.DATABASE_USER,
        password: process.env.DATABASE_PASSWORD,
        database: process.env.DATABASE
});

exports.login = async (req, res) => {
    try {

        const { email, password } = req.body;

        if ( !email || !password ) {
            return res.status(400).render('login', {
                message: 'You must enter your email or password'
            });
        }

        db.query('SELECT * FROM users WHERE email = ?', [email], async (error, result) => {
            console.log(result);
            if( !result || !(await bcrypt.compare(password, result[0].password))) {
                res.status(401).render('login', {
                    message: 'Email or Password is incorrect'
                })
            } else {
                const id = result[0].id;
                const jwtoken = jwt.sign({ id }, process.env.JWT_SECRET, {
                    expiresIn: process.env.JWT_EXPIRES_IN
                });

                console.log("The token for this session: " + jwtoken);
                const cookieOptions = { 
                    expires: new Date(
                        Date.now() + process.env.JWT_COOKIE_EXPIRES * 24 * 60 * 60 * 1000
                    ),
                    // Only let them from browser
                    httpOnly: true
                }

                res.cookie('jwt', jwtoken, cookieOptions);
                res.status(200).redirect("/");
            }
            
        })

    } catch (error) {
        console.log(error);
    }
}
exports.register = (req, res) => {
    console.log(req.body);
    const { name, email, password, passwordConfirm } = req.body;

    //En un futuro..
    //db.query("CREATE TABLE amigos (name VARCHAR(255), address VARCHAR(255))")

    // With ? we evade SQLinjection
    db.query('SELECT email FROM users WHERE email = ?', [email], async (error, result) => {

        if(error) {
            console.log(error);
        }
        
        if (result.length > 0) {
            return res.render('register', {
                message: 'That email is already in use'
            });

        } else if (password != passwordConfirm) {
            return res.render('register', {
                message: 'Password do not match'
            });
        }
        
        let hashedPassword = await bcrypt.hash(password, 8);
        console.log(hashedPassword);


    db.query('INSERT INTO users SET ?', {userName: name, email: email, password: hashedPassword}, (error, result) => {
        if (error) {
            console.log(error);
        } else {
            console.log(result);
            return res.render('register', {
                message: 'User registered'
            });
        }

    })

    });

}
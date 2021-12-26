import { hash } from 'bcrypt';
import { Request, Response } from 'express';
import * as jwt from 'jsonwebtoken';
import { getRepository } from 'typeorm';
import { User } from '../entity/User';
import config from '../config/config';
const bcrypt = require('bcrypt');


const userControllers = { 
    

    registerUser: async (req: Request, res: Response) => {

        // Cojemos el body.
        const newUser = getRepository(User).create(req.body);
        // Los nombres han de ser igual que en la DB.
        let { name, email, password, passwordConfirm, role } = req.body;
        const user = new User();

        user.name = name;
        user.password = password;
        user.role = role;

        
        // Comprobaciones en el body. 
        if ( name == null || email == null || password == null || passwordConfirm == null ) {
            
            return res.status(400).json( { message: "You must complete all camps !"});

        } else if (password !== passwordConfirm) {

            return res.status(400).json( { message: "Passwords do not match !"});
        }


        // Comprobar si el email existe en la DB.
        const check = await getRepository(User).findOne(
            { where: 
                { email }
            }
        );

        if (check) {
            return res.status(400).json({message: "This email already exist"})
        }

        getRepository(User).save(newUser);
        return res.status(200).json({message: "Succesfully registered !"})

    },

    login: async (req: Request, res: Response) => {

        const { email, password } = req.body;

        if (!(email && password)) {
            return res.status(400).json({message: ' Username & Password are required !'})
        }
        const userRepository = getRepository(User);
        let user: User;
        
        try {
            user = await userRepository.findOneOrFail({
                where: { 
                    email
                }
            });

        } catch (e) {

            return res.status(400).json({
                message: ' Username or password incorrect !'
            });
        }

        let checkPassword = await bcrypt.compare(password, user?.password);

        if (!checkPassword) return res.status(400).json('Invalid Password');

        if (!user?.active) return res.status(400).json('Account banned');

        const token = jwt.sign({userId: user.id, email: user.email}, config.jwtSecret, {expiresIn: '1h'});

        res.json({
            message: 'OK',
            token
        });

    }
}

export default userControllers;


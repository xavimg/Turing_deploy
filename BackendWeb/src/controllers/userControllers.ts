import { hash } from 'bcrypt';
import { Request, Response } from 'express';
import { getRepository } from 'typeorm';
import { User } from '../entity/User';
const bcrypt = require('bcrypt');


const userControllers = { 
    
    getUsers: async (req: Request, res: Response) => {

        // select * from ...
        const users = await getRepository(User).find();

        return res.json(users);

    },

    registerUser: async (req: Request, res: Response) => {

        // Cojemos el body.
        const newUser = getRepository(User).create(req.body);
        // Los nombres han de ser igual que en la DB.
        let { name, email, password, passwordConfirm, permission } = req.body;
        
        // Comprobaciones en el body. 
        if ( name == null || email == null || password == null || passwordConfirm == null || permission ==  null) {
            
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

        let checkUser = await getRepository(User).findOne(
            { where: 
                { email}
            }
        );

        const check = await bcrypt.compare(password, checkUser?.password);

        // const token = jwt.generate(credentials)

        if (check && checkUser?.active) {

            return res.status(200).send({
                "message": "Logged succesfully",
                "token": "token"})
        } else {
            return res.status(400).send({
                "message": "Your account has been banned"
                })
            }
        }
}

export default userControllers;


import { Request, Response } from 'express';
import { getRepository } from 'typeorm';
import { User } from '../entity/User';



const userControllers = { 
    
    getUsers: async (req: Request, res: Response) => {

        // select * from ...
        const users = await getRepository(User).find();

        return res.json(users);

    },

    registerUser: async (req: Request, res: Response) => {

        // Cojemos el body.
        const newUser = getRepository(User).create(req.body);
        const { nombre, email, password, passwordConfirm } = req.body;

        // Comprobaciones en el body.
        if ( !nombre || !email || !password || !passwordConfirm) {

            return res.status(400).json( { message: "You must complete all camps !"});

        } else if (password !== passwordConfirm) {

            return res.status(400).json( { message: "Passwords do not match !"});
        }

        // Comprobar si el mail existe en la DB.
        const check = await getRepository(User).findOne(
            { where: 
                { email }
            }
        );
        if (check) {

            return res.status(400).json({message: "This email already exist"})

        }

        //TO-DO [ hash password]

        getRepository(User).save(newUser);
        return res.status(200).json({message: "Succesfully registered !"})

    },

    login: async (req: Request, res: Response) => {

        const { email, password } = req.body;

        const credentials = await getRepository(User).findOne(
            { where: 
                { email, password }
            }
        );

        if (credentials) {
            return res.status(200).json({message: "Logged succesfully", token: "token"})
        }

    }
}

export default userControllers;


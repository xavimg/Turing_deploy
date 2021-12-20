import { hash } from 'bcrypt';
import { Request, Response } from 'express';
import { getRepository } from 'typeorm';
import { User } from '../entity/User';
const bcrypt = require('bcrypt');


const adminControllers = { 
    
    deleteUser: async (req: Request, res: Response) => {

        const { id } = req.body;

        await getRepository(User).delete(id);
    
        return res.status(200).send({
            "message": "User deleted !"
        });

    },
    banUser: async (req: Request, res: Response) => {

        const { id } = req.body;

        await getRepository(User).update(id, { active: false });
    
        return res.status(200).send({
            "message": "User banned !"
        })
    },
    UnbanUser: async (req: Request, res: Response) => {

        const { id } = req.body;

        await getRepository(User).update(id, { active: true });
    
        return res.status(200).send({
            "message": "User unbanned !"
        })
    }
    
}

export default adminControllers;


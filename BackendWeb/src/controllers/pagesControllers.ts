import { hash } from 'bcrypt';
import { Request, Response } from 'express';
import jwt from 'jsonwebtoken';
import { getRepository } from 'typeorm';
import { User } from '../entity/User';
const bcrypt = require('bcrypt');


const userControllers = { 
    
    profile: async (req: Request, res: Response) => {

        res.send('<p> Profile page </p>');
    },

}

export default userControllers;


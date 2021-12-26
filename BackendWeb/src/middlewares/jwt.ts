import { Request, Response, NextFunction } from 'express';
import * as jwt from 'jsonwebtoken';
import config from '../config/config';

export const checkJwt = (req: Request, res: Response, next: NextFunction) => {
    console.log('REQ->', req.headers);
    const token = <string> req.headers['auth'];
    let jwtPayload;
    
    try {

        jwtPayload = <any> jwt.verify(token, config.jwtSecret);
        res.locals.jwtPayload = jwtPayload;

    }
    catch (err) {

        return res.status(401).json({
            message: 'Token Unvalid'});
    }

    const {userId, name} = jwtPayload;

    const newToken = jwt.sign({userId, name}, config.jwtSecret, {expiresIn: '1h' });
    res.setHeader('token', newToken);
    // Call next function
    next();


}
import { Router } from 'express';
import userControllers from '../controllers/userControllers';
const router = Router();

// User routes
router.get('/login', userControllers.login)
router.post('/register', userControllers.registerUser)
router.get('/users' , userControllers.getUsers)

export default router;


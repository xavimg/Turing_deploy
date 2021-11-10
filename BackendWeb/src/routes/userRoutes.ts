import { Router } from 'express';
import userControllers from '../controllers/userControllers';
const router = Router();

router.get('/login', userControllers.login)
router.post('/register', userControllers.registerUser)
router.get('/users' , userControllers.getUsers)

// router.put('/users')
// router.delete('/users/:id')



export default router;


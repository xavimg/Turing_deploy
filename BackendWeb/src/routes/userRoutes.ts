import { Router } from 'express';
import userControllers from '../controllers/userControllers';
import pagesController from '../controllers/pagesControllers';
import adminControllers from '../controllers/adminControllers';
import { checkJwt } from '../middlewares/jwt';
import { checkRole } from '../middlewares/role';

const router = Router();

// User routes.
router.post('/login', userControllers.login)
router.post('/register', userControllers.registerUser)

// Pages routes.
router.get('/profile', [checkJwt, checkRole(['user','admin'])], pagesController.profile)

// admin routes.
router.get('/users', [checkJwt, checkRole(['admin'])] , adminControllers.getUsers);
router.delete('/user', [checkJwt, checkRole(['admin'])], adminControllers.deleteUser);
router.put('/user/ban',[checkJwt, checkRole(['admin'])], adminControllers.banUser);
router.put('/user/unban',[checkJwt, checkRole(['admin'])], adminControllers.UnbanUser);

export default router;

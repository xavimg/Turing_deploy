import { Router } from 'express';
import adminControllers from '../controllers/adminControllers';
const router = Router();

// Admin routes
router.delete('/user', adminControllers.deleteUser)
router.put('/user/ban', adminControllers.banUser)
router.put('/user/unban', adminControllers.UnbanUser)

export default router;


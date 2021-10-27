const express = require('express');
const router = express.Router();

const userController = require('../controllers/users');


router.get('/all', userController.getAllUsers);
router.get('/all/:id?', userController.getUserById);
router.get('/all/deleted/:id?', userController.deleteUserById);


module.exports = router;
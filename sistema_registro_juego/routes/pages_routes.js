const express = require('express');
const router = express.Router();

//const pagesController = require('../controllers/pages_controllers');

router.get('/', (req, res) => {
    res.render('index')
});

router.get('/register', (req, res) => {
    res.render('register')
});

module.exports = router;
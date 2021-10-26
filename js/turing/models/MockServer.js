class MockServer extends Server {

    constructor(token, baseuri) {
        super(token, baseuri);
        if (baseuri != "duke.io")
            throw new ServerException("server unreachable");
        if (token != "abcd1234")
            throw new ServerException("auth error");
    }

    getSystem(id) {
        if (id == 1) {
            return `
            {
                "system": {
                    "id": 1,
                    "size": {
                        "x": 4800,
                        "y": 4800
                    },
                    "planets": [
                        {
                        "id": 1,
                        "mass": 100,
                        "radius": 250,
                        "angularSpeed": 10,
                        "position": {
                            "x": 550,
                            "y": 550
                        },
                        "speed": {
                            "x": 0,
                            "y": 0
                        },
                        "name": "Earth",
                        "color": "0x997711",
                        "atmosphere": 
                            {
                                "layers": [
                                    {
                                    "radius": 300,
                                    "color": "0x0099DD",
                                    "alpha": 0.5
                                    },
                                    {
                                    "radius": 400,
                                    "color": "0x0099DD",
                                    "alpha": 0.2
                                    }
                                ]
                            }
                        }
                    ]
                }
            }
            `;
        } else return new ServerException("system not found");
    }

    getPlayer(id) {
        if (id == "self") {
            return `
            {
                "player" : {
                    "name": "Johaness Siesa",
                    "currentSystem": 1,
                    "currentPlanet": 0,
    
                    "position": {
                        "x": 55,
                        "y": 55
                    },

                    "rotation": 0,

                    "texture": "dat/spaceship.png"
                } 
            }
            `;
        } else {
            throw new ServerException("player not found");
        }
    }

}

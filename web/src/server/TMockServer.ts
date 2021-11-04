import { TServerI } from "./TServerI";

export class TMockServer implements TServerI {

    constructor() {

    }

    public createServer(token: string, baseuri: string): TServerI {
        return new TMockServer();
    }
    connected(): boolean {
        return true;
    }
    authorized(): boolean {
        return true;
    }

    getSystem(): string {
        const result : string = 
        `
        {
            "name": "Solar System",
            "id": "SOL-0000",
            "planets" : [
                {
                    "x": 0,
                    "y": 0,
                    "rotation": 0,
                    "radius": 500,
                    "color": "0x5555FF",
                    "alpha": 1,
                    "atmosphere" : {
                        "x": 0,
                        "y": 0,
                        "rotation": 0,
                        "radius": 700,
                        "color": "0x5555FF",
                        "alpha": 0.5,
                        "layers" : [
                            {
                                "x": 0,
                                "y": 0,
                                "rotation": 0,
                                "radius": 625,
                                "color": "0xFF00FF",
                                "alpha": 0.1
                            },
                            {
                                "x": 0,
                                "y": 0,
                                "rotation": 0,
                                "radius": 620,
                                "color": "0x0000FF",
                                "alpha": 0.1
                            },
                            {
                                "x": 0,
                                "y": 0,
                                "rotation": 0,
                                "radius": 610,
                                "color": "0x00FF00",
                                "alpha": 0.1
                            },
                            {
                                "x": 0,
                                "y": 0,
                                "rotation": 0,
                                "radius": 600,
                                "color": "0x00FFFF",
                                "alpha": 0.1
                            },
                            {
                                "x": 0,
                                "y": 0,
                                "rotation": 0,
                                "radius": 575,
                                "color": "0xFFFF00",
                                "alpha": 0.1
                            },
                            {
                                "x": 0,
                                "y": 0,
                                "rotation": 0,
                                "radius": 550,
                                "color": "0x0000FF",
                                "alpha": 0.5
                            }
                        ]
                    },
                    "name": "Earth",
                    "angularSpeed": 5
                }
            ]
        }
        `;
        return result;
    }
    getPlayer(): string {
        return "";
    }
    getSelfPlayer(): string {
        const result : string = 
        `
        `;
        return result;
    }
    
}
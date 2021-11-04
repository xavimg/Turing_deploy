import { TGraphic } from "./TGraphic";
import { TPlanet } from "./TPlanet";
import { TConstructedGraphicsI } from "./TConstrucedGraphicsI";
import { TDeserializableObjectI } from "./TDeserializableObjectI";

// Base Turing System class
export class TSystem extends TGraphic implements TConstructedGraphicsI, TDeserializableObjectI {

    protected name : string;
    protected id : string;
    protected planets : TPlanet[];
    
    constructor(name : string, id : string, parent? : TGraphic) {
        super(parent);
        this.name = name;
        this.id = id;
        this.planets = [];
    }

    static from(object : any, parent? : TGraphic) : TSystem {
        const result : TSystem = new TSystem(
            object.name, 
            object.id, 
            parent);
        object.planets.forEach((planet : any) => {
            result.planets.push(TPlanet.from(planet, result)) 
        });
        return result;
    }

    from(object : any, parent? : TGraphic): TSystem {
        return TSystem.from(object, parent);
    }

    withConstructedGraphics() : TSystem {
        this.planets!.forEach( (planet : TPlanet) => {
            this.addChild(planet.withConstructedGraphics());
        });
        return this;
    }

}
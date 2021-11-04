import { TGraphic } from "./TGraphic";
import { TElement } from "./TElement";
import { THealth } from "./THealth";
import { TDeserializableObjectI } from "./TDeserializableObjectI";
import { TConstructedGraphicsI } from "./TConstrucedGraphicsI";
import { resourceLimits } from "worker_threads";

export class TEntityException extends Error {}
export class TEntityExceptionNoSpriteSource extends TEntityException {}
export class TEntityExceptionSpriteSourceNotFound extends TEntityException {}

// Base Turing Entity class 
export class TEntity extends TElement implements TConstructedGraphicsI,TDeserializableObjectI{
    
    protected name : string;
    protected id : string;
    protected health : THealth;
    protected spriteSource? : string;

    constructor(x : number, y : number, width : number, height : number, rotation : number, health : THealth, name : string, id : string, parent? : TGraphic) {
        super(x, y, width, height, rotation, parent);
        this.health = health;
        this.name = name;
        this.id = id;
    }

    withConstructedGraphics(): TEntity {
        throw new Error("Method not implemented.");
    }

    static from(object : any, parent : TGraphic) : TEntity {
        const result : TEntity = new TEntity(object.x, object.y, object.width, object.height, object.rotation, THealth.from(object.health, parent), object.name, object.id, parent);
        return result;
    }

    from(object: any): TEntity {
        throw new Error("Method not implemented.");
    }

}
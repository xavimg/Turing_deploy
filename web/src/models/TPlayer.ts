import { TEntity } from "./TEntity";
import { THealth } from "./THealth";
import { TGraphic } from "./TGraphic";

// Base Turing Player class
export class TPlayer extends TEntity {
    
    constructor(x : number, y : number, width : number, height : number, rotation : number, health : THealth, name : string, id : string, parent? : TGraphic) {
        super(x, y, width, height, rotation, health, name, id, parent);
    }
    
}
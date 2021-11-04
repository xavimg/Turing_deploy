import { TCircle } from "./TCircle";
import { TAtmosphere } from "./TAtmosphere";

import { Graphics } from "@pixi/graphics";
import { TGraphic } from "./TGraphic";
export class TPlanet extends TCircle {

    protected atmosphere : TAtmosphere;
    protected name : string;
    protected angularSpeed : number;

    constructor(x : number, y : number, rotation : number, radius : number, color : number, alpha : number, atmosphere : TAtmosphere, name : string, angularSpeed : number, parent? : TGraphic) {
        super(x, y, rotation, radius, color, alpha, parent);
        this.atmosphere = atmosphere;
        this.name = name;
        this.angularSpeed = angularSpeed;
    }

    static from(object : any, parent? : TGraphic) : TPlanet {
        return new TPlanet(object.x, object.y, object.rotation, object.radius, object.color, object.alpha, TAtmosphere.from(object.atmosphere, parent), object.name, object.angularSpeed, parent);
    }

    withConstructedSprite() : TPlanet {
        const graphics = new Graphics();
        graphics.beginFill(this.color, this.alpha);
        graphics.drawCircle(this.x, this.y, this.radius);
        graphics.endFill();
        this.sprite.addChild(graphics);
        debugger;
        return this;
    }

}
import { TGraphic } from "./TGraphic";
import { TShape } from "./TShape";
import { TConstructedGraphicsI } from "./TConstrucedGraphicsI";
import { TDeserializableObjectI } from "./TDeserializableObjectI";

import { Graphics } from "@pixi/graphics";

export class TCircle extends TShape implements TConstructedGraphicsI, TDeserializableObjectI {

    protected radius : number;
    
    constructor(x : number, y : number, rotation : number, radius : number, color : number, alpha : number, parent? : TGraphic) {
        super(x, y, (radius * 2), (radius * 2), rotation, color, alpha, parent);
        this.radius = radius;
    }
    
    getRadius() {
        return this.radius;
    }

    setRadius(radius : number) {
        this.radius = radius;
    }

    static from(object : any, parent? : TGraphic) : TCircle {
        return new TCircle(object.x, object.y, object.rotation, object.radius, object.color, object.alpha, parent);
    }

    from(object : any, parent? : TGraphic) : TCircle {
        return TCircle.from(object, parent);
    }

    withConstructedGraphics() : TCircle {
        const graphics : Graphics = new Graphics();
        graphics.beginFill(this.color, this.alpha);
        graphics.drawCircle(this.x, this.y, this.radius);
        graphics.endFill();
        this.sprite = graphics;
        return this;
    }

}
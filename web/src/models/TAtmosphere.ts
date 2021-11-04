import { TGraphic } from "./TGraphic";
import { TCircle } from "./TCircle";
import { TConstructedGraphicsI } from "./TConstrucedGraphicsI";
import { TDeserializableObjectI } from "./TDeserializableObjectI";

export class TAtmosphere extends TCircle implements TConstructedGraphicsI, TDeserializableObjectI {

    protected layers : TCircle[];

    constructor(x : number, y : number, radius : number, rotation : number, color : number, alpha : number, layers : TCircle[], parent? : TGraphic) {
        super(x, y, radius, color, alpha, rotation, parent);
        this.layers = layers;
    }

    static from(object : any, parent? : TGraphic) : TAtmosphere {
        return new TAtmosphere(object.x, object.y, object.rotation, object.radius, object.color, object.alpha, object.layers.map( (layer : any) => TCircle.from(layer, parent)), parent);
    }

    from(object : any, parent? : TGraphic) {
        return TAtmosphere.from(object, parent);
    }

    withConstructedGraphics() : TAtmosphere {
        this.layers.forEach( (layer : TCircle) => {
            this.addChild(layer.withConstructedGraphics())
        });
        return this;
    }

}
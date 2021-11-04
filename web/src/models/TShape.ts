import { TGraphic } from "./TGraphic";
import { TElement } from "./TElement";

export class TShape extends TElement {
    
    protected color : number;
    protected alpha : number;

    constructor(x : number, y : number, width : number, height : number, rotation : number, color : number, alpha : number, parent? : TGraphic) {
        super(x, y, width, height, rotation, parent);
        this.color = color;
        this.alpha = alpha;
    }

    getColor() {
        return this.color;
    }

    setColor(color : number) {
        this.color = color;
    }

    getAlpha() {
        return this.alpha;
    }

    setAlpha(alpha : number) {
        this.alpha = alpha;
    }

}
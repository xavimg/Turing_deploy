import { TGraphic } from "./TGraphic";

// Base Turing Element class 
export abstract class TElement extends TGraphic {

    protected x : number;
    protected y : number;
    protected width : number;
    protected height : number;
    protected rotation : number;

    constructor(x : number, y : number, width : number, height : number, rotation : number, parent? : TGraphic) {
        super(parent);
        this.x = x;
        this.y = y;
        this.width = width;
        this.height = height;
        this.rotation = rotation;
    }

    getX() {
        return this.x;
    }

    setX(x : number) {
        this.x = x;
    }

    getY() {
        return this.y;
    }

    setY(y : number) {
        this.y = y;
    }

    getWidth() {
        return this.width;
    }

    setWidth(width : number) {
        this.width = width;
    }

    getHeight() {
        return this.height;
    }

    setHeight(height : number) {
        this.height = height;
    }

    getRotation() {
        return this.rotation;
    }

    setRotation(rotation : number) {
        this.rotation = rotation;
    }

}
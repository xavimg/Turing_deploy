import { TObject } from "./TObject";

export class THealth extends TObject {

    private maxValue : number;
    private value : number;
    private god : boolean;

    constructor(maxValue : number, value : number = maxValue, god : boolean = false, parent : TObject) {
        super(parent);
        this.maxValue = maxValue;
        this.value = value;
        this.god = god;
    }

    static from(object : any, parent : TObject) {
        return new THealth(object.maxValue, object.value, object.god, parent);
    }

    getValue() {
        return this.value;
    }

    getMaxValue() {
        return this.maxValue;
    }

    isGod() {
        return this.god;
    }

    damage(amount : number) {
        if (!this.god)
            this.value -= amount;
    }

    heal(amount : number) {
        if (!this.god)
            this.value += amount;
    }


}
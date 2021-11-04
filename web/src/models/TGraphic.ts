import { Container } from "@pixi/display";
import { TObject } from "./TObject";

export abstract class TGraphic extends TObject {

    protected sprite : Container;

    constructor(parent? : TGraphic) {
        super();
        this.sprite = new Container();
        if (parent != undefined)
            this.setParent(parent!);
    }

    getSprite() : Container {
        return this.sprite;
    }

    setSprite(sprite : Container) {
        this.sprite = sprite;
    }

    override getParent() : TGraphic {
        return ((this.parent!) as TGraphic);
    }

    override setParent(parent : TGraphic) {
        this.parent = parent;
        parent.addChild(this);
    }

    override getChilds() : TGraphic[] {
        return ((this.childs!) as TGraphic[]);
    }

    override addChild(child : TGraphic) {
        this.childs!.push(child);
        this.sprite.addChild(child.getSprite());
    }

}
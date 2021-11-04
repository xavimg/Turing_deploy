// Base Turing Class Object
export class TObject {
    
    protected debug : boolean = false;
    protected parent? : TObject;
    protected childs? : TObject[];

    constructor(parent? : TObject) {
        this.parent = parent;
        this.childs = [];
    }

    getDebug() : boolean {
        return this.debug;
    }

    setDebug(debug : boolean) {
        this.debug = debug;
    }

    getParent() : TObject {
        return this.parent!;
    }

    setParent(parent : TObject) {
        this.parent = parent;
    }

    getChilds() : TObject[] {
        return this.childs!;
    }

    addChild(child : TObject) {
        this.childs!.push(child);
    }
    
}
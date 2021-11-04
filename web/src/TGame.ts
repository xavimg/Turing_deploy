import { Application, IApplicationOptions } from 'pixi.js';
import { TConstructedGraphicsI } from './models/TConstrucedGraphicsI';
import { TGraphic } from './models/TGraphic';
import { TPlayer } from './models/TPlayer';
import { TSystem } from './models/TSystem';
import { TServerI } from './server/TServerI';

export class TGame extends TGraphic implements TConstructedGraphicsI {
    
    public server : TServerI;
    public app : Application;
    public player? : TPlayer;
    public system? : TSystem;

    public constructor(server : TServerI, appOptions : IApplicationOptions = {width: window.innerWidth, height: window.innerHeight, backgroundColor: 0}) {
        super();
        this.app = new Application(appOptions);
        this.app.stage.addChild(this.sprite);
        this.server = server;
    }

    withConstructedGraphics() : TGame {
        this.addChild(this.system!.withConstructedGraphics());
        return this;
    }

    updateSystem() : TGame {
        this.system = TSystem.from(JSON.parse(this.server.getSystem()), this);
        return this;
    }
    
}
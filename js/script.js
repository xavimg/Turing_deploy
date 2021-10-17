const marquina = 
`
Catalunya, comtat gran,
qui t'ha vist tan rica i plena!
Ara el rei Nostre Senyor
declarada ens te la guerra.

Segueu arran!
Segueu arran,
que la palla va cara!
Segueu arran! 

Lo gran comte d'Olivars
sempre li burxa l'orella:
-Ara es hora, nostre rei,
ara es hora que fem guerra.- 

Contra tots els catalans,
ja veieu quina n'han feta:
seguiren viles i llocs
fins al lloc de Riu d'Arenes; 

n'han cremat un sagrat lloc,
que Santa Coloma es deia;
cremen albes i casulles,
i caporals i patenes,
i el Santíssim Sagrament,
alabat sia per sempre. 

Mataren un sacerdot,
mentre que la missa deia;
mataren un cavaller,
a la porta de l'església,
en Lluís de Furrià,
i els àngels li fan gran festa.

Lo pa que no era blanc
deien que era massa negre:
el donaven als cavalls
sols per assolar la terra. 

Del vi que no era bo,
n'engegaven les aixetes,
el tiraven pels carrers
sols per regar la terra. 

A presencia dels parents
deshonraven les donzelles.
Ne donen part al Virrei,
del mal que aquells soldats feien: 

-Llicència els he donat jo,
molta més se'n poden prendre.-

Sentint resposta semblant,
enarboren la bandera;
a la plaça de Sant Jaume,
n´hi foren les dependències. 

A vista de tot això
s'és avalotat la terra:
comencen de llevar gent
i enarborar les banderes. 

Entraren a Barcelona
mil persones forasteres;
entren com a segadors,
com érem en temps de sega.

De tres guàrdies que n'hi ha,
ja n'han morta la primera;
ne mataren al Virrei,
a l'entrant de la galera;
mataren els diputats
i els jutges de l'Audiència. 

Aneu alerta, catalans;
catalans, aneu alerta:
mireu que aixís ho faran,
quan seran en vostres terres. 

Anaren a la presó:
donen llibertat als presos.
El bisbe els va beneir
Amb la ma dreta i l'esquerra: 

-On es vostre capità?
On és vostre bandera?-
Varen treure el bon Jesús
Tot cobert amb un vel negre: 

-Llicència els he donat jo,
molta més se'n poden prendre.-

Sentint resposta semblant,
enarboren la bandera;
a la plaça de Sant Jaume,
n´hi foren les dependències. 

A vista de tot això
s'és avalotat la terra:
comencen de llevar gent
i enarborar les banderes. 

Entraren a Barcelona
mil persones forasteres;
entren com a segadors,
com érem en temps de sega.

De tres guàrdies que n'hi ha,
ja n'han morta la primera;
ne mataren al Virrei,
a l'entrant de la galera;
mataren els diputats
i els jutges de l'Audiència. 

Aneu alerta, catalans;
catalans, aneu alerta:
mireu que aixís ho faran,
quan seran en vostres terres. 

Anaren a la presó:
donen llibertat als presos.
El bisbe els va beneir
Amb la ma dreta i l'esquerra: 

-On es vostre capità?
On és vostre bandera?-
Varen treure el bon Jesús
Tot cobert amb un vel negre: 

-Aquí és nostre capità,
aquesta es nostre bandera.-
A les armes catalans,
Que ens ha declarat la guerra! 

Segueu arran!
Segueu arran,
que la palla va cara!
Segueu arran!
`;

const app = new PIXI.Application({
    width: window.innerWidth,
    height: window.innerHeight
});

document.body.appendChild(app.view);

class Element {
    randomizeposition(w, h) {
        this.sprite.position.x = Math.random() * w;
        this.sprite.position.y = Math.random() * h;
    }

    randomizescale(max) {
        const scale = Math.random() * max;
        this.sprite.scale.x = scale;
        this.sprite.scale.y = scale;
    }
    randomizerotation(max) {
        const scale = Math.random() * max;
        this.sprite.rotation = scale;
    }
    randomizetint(amount) {
        this.sprite.tint = Math.random() * amount;
    }
}


class Star extends Element {
    constructor() {
        super();
        this.texture = PIXI.Texture.from("dat/star.png");
        this.sprite = new PIXI.Sprite(this.texture);
    }
}

class Planet extends Element {
    constructor() {
        super();
        this.texture = PIXI.Texture.from("dat/planet1.png");
        this.sprite = new PIXI.Sprite(this.texture);
    }
    
    randomizetexture() {
        const paths = ["planet1.png", "planet2.png"].map( x => "dat/" + x);
        this.texture = PIXI.Texture.from(paths[Math.round(Math.random())]);
        this.sprite = new PIXI.Sprite(this.texture);
    }

}

class Map {
    constructor(width, height) {
        this.container = new PIXI.Container();
        this.width = width;
        this.height = height;
        this.createstars(2000);
        this.createplanets(2);
    }

    createstars(amount) {
        for (let i = 0; i < amount; i++) {
            let star = new Star();
            star.randomizeposition(this.width, this.height);
            star.randomizescale(0.15);
            star.randomizetint(0xFFFFFF);
            this.container.addChild(star.sprite);
        }
    }

    createplanets(amount) {
        for (let i = 0; i < amount; i++) {
            let planet = new Planet();
            planet.randomizetexture();
            planet.randomizeposition(this.width, this.height);
            planet.randomizescale(0.5);
            planet.randomizerotation(360);
            planet.randomizetint(0x111111);
            this.container.addChild(planet.sprite);
        }
    }
}

/**
 * Game idea: randomizetint(0) creates a black void. This black voids could be "black holes" that the player can feel gravitational pull towards but has to dodge.
 */

class SuperMap {
    
    constructor() {
        this.chunks = [];
    }

    updateChunk(position, chunk) {
        switch (position) {
            case "n": {
                this.chunks.n = chunk;
            } break;
            
            case "r": {
                this.chunks.r = chunk;
            } break;
            
            case "d": {
                this.chunks.d = chunk;
            } break;
                
            case "l": {
                this.chunks.l = chunk;
            } break;
        }
    }
}

let map = new Map(app.screen.width, app.screen.height);

app.stage.addChild(map.container);

function loop() {
    while (true) {
    }
}
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

class RandomElement {

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

    randomizetint(amount, min, chance) {
        if ((Math.random() * 100) < chance)
            this.sprite.tint = (Math.random() * amount) + min;
    }

}


class Star extends RandomElement {

    constructor() {
        super();
        this.texture = PIXI.Texture.from("dat/star.png");
        this.sprite = new PIXI.Sprite(this.texture);
    }

}

class Planet extends RandomElement {

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
        this.createstars(Math.pow(2, 16));
        this.createplanets(Math.pow(2, 4));
    }

    createstars(amount) {
        for (let i = 0; i < amount; i++) {
            let star = new Star();
            star.randomizeposition(this.width, this.height);
            star.randomizescale(0.15);
            star.randomizetint(0x55FFFF, 0xAA0000, 10); //a few red-shifted tints
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
            planet.randomizetint(0xFFFFFF, 0x0, 100);
            this.container.addChild(planet.sprite);
        }
    }

    move(x, y) {
        this.container.position.x += x;
        this.container.position.y += y;
    }

}

/**
 * Game idea: randomizetint(0) creates a black void. 
 * This "black voids" could be "black holes" 
 * The player can feel gravitational pull towards them to but would have to dodge.
 */

class Game {

    constructor() {
        this.map = new Map(Math.pow(2, 12), Math.pow(2, 12));
    }

    logKey(e) {
        switch (e.keyCode) {
            case 87: {
                console.log("w");
                turing.map.move(0, 5);
            } break;
            case 83: {
                console.log("S");
                turing.map.move(0, -5);
            } break;
            case 65: {
                console.log("A");
                turing.map.move(5, 0);
            } break;
            case 68: {
                console.log("D");
                turing.map.move(-5, 0);
            } break;

            default: {
                console.log(e.keyCode);
            } break;
        }
    }

}

const app = new PIXI.Application({
    width: window.innerWidth,
    height: window.innerHeight
});
document.body.appendChild(app.view);

let turing = new Game();
app.stage.addChild(turing.map.container);

document.addEventListener('keydown', turing.logKey);
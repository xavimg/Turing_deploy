export class TLogger {

    public static enabled : boolean = true;
    public static level : number = 0;

    static log(level : number = 3, ...message: string[]) {
        if (level >= this.level && TLogger.enabled) {
            console.log(`Called logger on level ${level}:\n`, message.join(""));
        }
    }

}
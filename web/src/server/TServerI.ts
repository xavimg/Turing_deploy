export interface TServerI {
    createServer(token : string, baseuri : string) : TServerI;
    connected(): boolean;
    authorized(): boolean;

    getSystem(): string;
    getPlayer(): string;
    getSelfPlayer(): string;
}
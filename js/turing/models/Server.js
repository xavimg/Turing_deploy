class Server {

    constructor(token, baseuri) {
        this.headers = [
            {'Authorization': token}
        ];
        this.baseuri = baseuri;
    }

    httpRequest(url, type, body) {
        //
    }

    getRequest(path) {
        //
    }

    postRequest(path, body) {
        //
    }

}

class ServerException extends Error {}
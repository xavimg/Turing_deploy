class TuringConnection {

    constructor(baseuri, headers) {
        this.baseuri = baseuri;
        this.headers = headers;
    }

    // POST Requests

    async httpPOSTRequest(url, data) {
        const response = await fetch(url, {
            method: 'POST',
            headers: this.headers,
            body: data,
        });
        return response;
    }

    httpPOSTRequestWithBaseURI(at, data) {
        return httpPOSTRequest(`${this.baseurl}${at}`, data);
    }

    // GET Requests

    async httpGETRequest(url) {
        const response = await fetch(url, {
            method: 'GET',
            headers: this.headers
        });
        return response;
    }

    httpGETRequestWithBaseURI(at) {
        return httpGETRequest(`${this.baseurl}${at}`);
    }

    // Auth utils

    /**
     * returns session token or null
     */

    getUserToken() {
        return JSON.parse(this.httpGETRequestWithBaseURI('/player/auth').json()).sessiontoken;
    }

    generateAuthHeaders() {
        this.headers = {
            "Authorization": this.getUserToken()
        };
    }

    // Api functions

    getUser(id) {
        return this.httpGETRequestWithBaseURI(`/player/${id}`);
    }

    getUserSelf() {
        return this.getUser("me");
    }

    getChunk(x, y) {
        return this.httpGETRequestWithBaseURI(`/chunk/${x}/${y}`);
    }

}
# Docker setup
To initialize the docker containers, run ```docker-compose up```.
When doing so, ```docker-compose``` will look instide your local ```.env``` file for the following variables:

- **TURING_USERNAME**: Username for the MongoDB connection (it can be whatever you want)
- **TURING_PASSWORD**: Password for the MongoDB connection (it can be whatever you want)
- **TURING_DATABASE**: Database for the MongoDB connection (it can be whatever you want)

The MongoDB service will be fowarded to port **1234** and the server's service to port **8080**

> **NOTE**
> 
> By release time, the MongoDB service will not be forwarded to any port, and it will be managed entirelly by the server's service

Also, the server will need you to pass the ```.env``` variable **JWT_SECRET** to be able to decrypt JWT tokens
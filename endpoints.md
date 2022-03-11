# Generic
## GET /status
Response:
```json 
{
    "running": bool,
    "database": bool
}
```

## GET /resources
Reponse:
```json
{
    "resources": [{
        "name": string,
        "size": single,
        "type": string?
    }]
}
```

# Player
## GET /player
Response:
```json
{
    "_id": objectId,
    "name": string,
    "location": {
        "system": object,
        "pos": {
            "x": double,
            "y": double
        }
    },
    "hp": uint,
    "stats": {
        "level": uint,
        "max_speed": double,
        "max_hp": double
    },
    "inventory": [object],
    "color": uint
}
```

## GET /player/{:id}
Response:
```json
{
    "_id": objectId,
    "name": string,
    "system": object,
    "hp": uint,
    "level": uint,
    "color": uint
}
```

## POST /player/signup
Body: Frontend ID

## POST /player/signin
Header:
```json
{
    "Authorization": Bearer {string}
}
```

## POST /player/signout
Header:
```json
{
    "Authorization": Bearer {string}
}
```

# Planetary System
## (TODO) GET /system
```json
{
    "_id": objectId,
    "minLevel": uint,
    "players": [object],
    "star": [{
        "mass": double,
        "rotation": double
    }],
    "planets": [{
        "_id": uint,
        "name": string,
        "color": uint,
        "pos": {
            "x": double,
            "y": double
        },
        "vel": {
            "x": double,
            "y": double
        }
    }]
}
```

# (TODO) GET /system/update
```json
{
    "_id": objectId,
    "players": [object],
    "planets": [{
        "_id": uint,
        "name": string,
        "color": uint,
        "pos": {
            "x": double,
            "y": double
        },
        "vel": {
            "x": double,
            "y": double
        }
    }]
}
```

## (TODO) GET /system/{:id}
Obtain system info
## /signup
## /signin
## /signout

# Player

## GET /player
```json
{
    "_id": objectId,
    "name": string,
    "system": object,
    "hp": uint,
    "stats": {
        "level": uint,
        "max_speed": double,
        "max_hp": double
    },
    "pos": {
        "x": double,
        "y": double
    },
    "inventory": [object],
    "color": uint
}
```

## GET /player/{:id}
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

# Planetary System

## GET /system
```json
{
    "_id": objectId,
    "minLevel": uint,
    "users": [object],
    "star": [{
        "mass": double,
        "rotation": double
    }],
    "planets": [{
        "_id": uint,
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

## GET /system/{:id}
Obtain system info
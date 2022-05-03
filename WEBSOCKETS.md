# Base Payload
```json
{
    "id": uint,
    "body": object
}
```

# Payload Bodies
## Client
### Player Update (0x00)
```json
{
    "system": objectId?,
    "dir": float, // degrees
    "at": ulong, // fimestamp
    "position": {
        "x": double,
        "y": double
    }
}
```

## Server
### Player Update (0x10)
```json
{
    "player": objectId,
    "position": {
        "x": double,
        "y": double
    }
}
```

### New Player (0x11)
```json
{
    "id": objectId,
    "name": string,
    "location": {
        "system": ObjectId,
        "position": {
          "x": double,
          "y": double
        }
    },
    "color": uint
}
```

### Current status (0x12)
```json
{
    "system": objectId,
    "position": {
        "x": double,
        "y": double
    },
    "players": [
        {
            "id": objectId,
            "name": string,
            "location": {
                "system": ObjectId,
                "position": {
                    "x": double,
                    "y": double
                }
            },
            "color": uint
        }
    ]
}
```
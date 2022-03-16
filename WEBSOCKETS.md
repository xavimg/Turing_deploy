# Base Payload
```json
{
    "id": uint,
    "body": object
}
```

# Payload Bodies
## Client Player Update (0x00)
```json
{
    "system": objectId,
    "x": double,
    "y": double
}
```

## Server Player Update (0x10)
```json
{
    "player": objectId,
    "x": double,
    "y": double
}
```
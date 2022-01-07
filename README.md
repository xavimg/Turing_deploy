# Turing

## Dockerfile
Build the dockerfile with\
```docker build --build-arg TURING_DATABASE={db} --build-arg TURING_USERNAME={username} --build-arg TURING_PASSWORD={password} -t aandreba/turing . && docker run -p 27017:27017 <image_tag> ```
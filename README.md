# Lipoic-Backend
[![](https://img.shields.io/github/license/Lipoic/Lipoic-Frontend.svg)](LICENSE)
[![Rocket Homepage](https://img.shields.io/badge/web-rocket.rs-red.svg?style=flat&label=https&colorB=d33847)](https://rocket.rs)
![MongoDB](https://img.shields.io/badge/MongoDB-%234ea94b.svg??style=flat-square&logo=mongodb&logoColor=white)

## Deployment
Use Docker deploy Lipoic-Backend.
1. Install [docker-compose](https://docs.docker.com/compose/install/).
2. Run [`./genrsa.sh`](./genrsa.sh) generate RSA private key and public key.
3. Change [`docker-compose.yml`](./docker-compose.yml) environment to your.
4. Run [`./deploy.sh`](./deploy.sh).

## Contribute
1. [Fork](https://docs.github.com/en/get-started/quickstart/fork-a-repo) this [repository](https://github.com/Lipoic/Lipoic-Server) to your own GitHub account and then [clone](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository) it to your local device
2. Install [docker-compose](https://docs.docker.com/compose/install/)
3. Use Docker deploy mongodb `docker-compose -f ./mongodb/docker-compose.yml up -d`
4. To run `cargo run`
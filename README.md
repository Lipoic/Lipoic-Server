# Lipoic-Server

[![](https://img.shields.io/github/license/Lipoic/Lipoic-Server.svg)](LICENSE)
[![Rocket Homepage](https://img.shields.io/badge/web-rocket.rs-red.svg?style=flat&label=https&colorB=d33847)](https://rocket.rs)
![MongoDB](https://img.shields.io/badge/MongoDB-%234ea94b.svg??style=flat-square&logo=mongodb&logoColor=white)

## Deployment
Use Docker deploy Lipoic Server.
1. Install [docker-compose](https://docs.docker.com/compose/install/).
2. Run [`./script/genrsa.sh`](./script/genrsa.sh) generate RSA private key and public key.
3. Change [`Rocket.toml`](./Rocket.toml) variables to your.
4. Run [`./script/deploy.sh`](./script/deploy.sh).

Note: If you are using Windows or macOS use [cygwin](https://www.cygwin.com) to run the script

## Contribute
1. [Fork](https://docs.github.com/en/get-started/quickstart/fork-a-repo) this [repository](https://github.com/Lipoic/Lipoic-Server) to your own GitHub account and then [clone](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository) it to your local device
2. Install [docker-compose](https://docs.docker.com/compose/install/)
3. Run the command
```shell
docker-compose -f ./script/mongodb/docker-compose.yml up -d
```
4. To run `cargo run`
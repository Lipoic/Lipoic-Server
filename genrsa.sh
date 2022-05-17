# /bin/bash

openssl genrsa -out privatekey.pem 2048
openssl rsa -in privatekey.pem -pubout > publickey.pem
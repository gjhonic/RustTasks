@recipes:
    just --list --unsorted

up:
    sudo docker-compose up

curl:
    curl -v http://127.0.0.1:3030/hello
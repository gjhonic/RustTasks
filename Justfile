#Рецепты блюд
@recipes:
    echo ' ____            _  _____         _        '
    echo '|  _ \ _   _ ___| ||_   _|_ _ ___| | _____ '
    echo '| |_) | | | / __| __|| |/ _` / __| |/ / __|'
    echo '|  _ <| |_| \__ \ |_ | | (_| \__ \   <\__ \'
    echo '|_| \_\\\__,_|___/\__||_|\__,_|___/_|\_\___/'
    echo '                                 RustTasks '
    just --list --unsorted

#Подготавливаем проект
cook:
    @echo 'Create variable'
    cp .env.sample .env

#Запускает cargo run
bum:
    @echo 'Start local run project'
    cargo run

#Собирает и запускает докер
up:
    sudo docker-compose up

#Делает тестовый запрос на сервер
test-curl:
    curl -v http://127.0.0.1:3030/test
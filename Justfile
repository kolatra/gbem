set dotenv-load

alias r := run

da:
    cargo r --release -p disassembler -- -f ./roms/snake.gb

run:
    cargo r --release -p front

lint:
    ./lint.sh

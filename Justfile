set dotenv-load

alias r := run

da:
    cargo r --release -p disassembler

run:
    cargo r --release -p front

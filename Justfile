set dotenv-load

alias r := run

da:
    cargo r --release -p disassembler -- -f ./test-roms/blargg/mem_timing/mem_timing.gb

run:
    cargo r --release -p front

lint:
    ./lint.sh

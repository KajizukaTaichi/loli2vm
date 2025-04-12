cargo run > example.asm
nasm -f macho64 example.asm -o example.o
clang -o example example.o -e _start
./example
echo $?

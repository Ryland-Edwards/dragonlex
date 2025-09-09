.PHONY: build run test clean

build:
	cargo build --release
	copy target\release\dragonlex.exe dragonlex.exe

run: build
	.\dragonlex.exe Levi.spec
	rustc lexer.rs -o lexer.exe

test: run
	.\lexer.exe test.txt > test.tokens

clean:
	cargo clean
	-del /Q dragonlex.exe lexer.exe lexer.rs test.tokens 2>nul

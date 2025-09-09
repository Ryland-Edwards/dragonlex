to run with rust installed use: \
cargo build \
cargo run drag.spec \
./lexer test.txt > test.tokens

or run included binary for x86_64-unknown-linux-gnu: \
./dragonlex drag.spec \
./lexer test.txt > test.tokens 

the terminal will print bad input but ignore that as those are just spaces that i didnt put in the grammar
cargo build
cargo run &
LSP_PID=$!

echo $LSP_PID

exit 0
# Take user input here to block...

cat server_init_rpc_1.txt


sed -u -re 's/^(.*)$/\1\r/' |

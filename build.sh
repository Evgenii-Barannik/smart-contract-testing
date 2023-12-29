#!/bin/bash
substrate-contracts-node & SUB_SERVER_PID=$!
cargo contract build
cargo contract upload --suri //Alice --execute

CONTRACT=$(cargo contract instantiate --execute --suri //Bob --args true --skip-confirm |
perl -nle 'print $1 if /Contract\s+(\w+)/')

echo "The contract address is: $CONTRACT"
cargo contract call --contract $CONTRACT --message grant_minter_role --suri //Alice --skip-confirm --execute
cargo contract call --contract $CONTRACT --message mint_token --args 11 --suri //Alice  --skip-confirm --execute
cargo contract call --contract $CONTRACT --message mint_token --args 12 --suri //Bob  --skip-confirm --execute

# Disabling the server
kill $SUB_SERVER_PID
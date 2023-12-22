#!/bin/bash
substrate-contracts-node & SUB_SERVER_PID=$!
cargo contract build
cargo contract upload --suri //Alice

CONTRACT=$(cargo contract instantiate --execute --suri //Alice --args true --skip-confirm |
perl -nle 'print $1 if /Contract\s+(\w+)/')

echo "The contract address is: $CONTRACT"
cargo contract call --contract $CONTRACT --message get --suri //Alice --skip-confirm
cargo contract call --contract $CONTRACT --message flip --execute --suri //Alice --skip-confirm
cargo contract call --contract $CONTRACT --message get --suri //Alice --skip-confirm 
#!/bin/sh
dir="/home/dni/.lightning"
datadir="$dir/regtest/data"
node_list="1 2 3 4 5 6"
nodes=6

bcli() {
  docker exec balanzierer-bitcoind-1 bitcoin-cli -rpcuser=balanzierer -rpcpassword=balanzierer -regtest $@
}

lcli() {
  # args(i, cmd)
  echo "running cmd: $2 on node: $1"
  docker exec balanzierer-clightning-$1 lightning-cli --network regtest $2
}

lcli_all () {
  for i in $node_list; do
    lcli $i $1
  done
}

docker_start() {
  docker-compose up --scale clightning=$nodes -d
}

init() {
  init_bitcoin_wallet
  sleep 3
  fund_lightning_wallets
  sleep 2
}

init_bitcoin_wallet() {
  echo "init_bitcoin_wallet..."
  bcli createwallet balanzierer || bcli loadwallet balanzierer
  bcli -generate 150
}

fund_lightning_wallets() {
  echo "fund_lightning_wallets..."
  for i in $node_list; do
    address=$(lcli $i newaddr | jq -r .bech32)
    echo "funding wallet: $address on node: $i"
    bcli -named sendtoaddress address=$address amount=10 fee_rate=100
  done
  bcli -generate 1
}

create_channels() {
  echo "create_channels..."
  bcli -generate 1
}


balance() {
 run_all
}

unbalance() {
 run_all
}

force_close() {
 run_all
}

$@

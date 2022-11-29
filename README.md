## Simple example for sending and listening through 'ethers-rs'

### plelase follow the way to run this example:
1. clone https://github.com/ashuralyk/ibc-solidity-contract into local directory
2. make sure `Node.js` has been installed
3. run `npm i` to install packages
4. run `npx hardhat node` to start a local Ethereum network
5. open a new terminal and run `npx hardhat run ./scripts/deploy.js` to deploy IBC contracts on the node
6. run `cargo run` to send and listen via ethers-rs
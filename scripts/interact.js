const { SigningCosmWasmClient } = require("@cosmjs/cosmwasm-stargate");
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { GasPrice } = require("@cosmjs/stargate");
require("dotenv").config();

// Configuration
const RPC_ENDPOINT = "https://rpc.xion-testnet-2.burnt.com:443";
const MNEMONIC = process.env.MNEMONIC;
const ESCROW_CONTRACT_ADDRESS = "xion1t6pp2v9fz5lsdl5ru9ynwcs322qrqe7hjpj66cqsmfjwfkukel8sesxm8y"
const USDC_CONTRACT_ADDRESS = process.env.USDC_CONTRACT_ADDRESS;
const CHAIN_ID = process.env.CHAIN_ID || "xion-testnet-2";
const GAS_PRICE = GasPrice.fromString("0.025uxion");

// Main function to run interactions
async function main() {
  // 1. Initialize wallet and client
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(MNEMONIC, {
    prefix: "xion",
  });
  
  const [account] = await wallet.getAccounts();
  const client = await SigningCosmWasmClient.connectWithSigner(
    RPC_ENDPOINT,
    wallet,
    { gasPrice: GAS_PRICE }
  );
   
  console.log("Connected with address:", account.address);
  
  // // 2. Approve USDC spending (required before initiating escrow)
  // await approveUSDC(client, account);
  
  // 3. Initiate escrow
  const seller = "xion1c6y8tdknd5qpkxtph9l0njj0dxdzglwrw4f3de"; // Replace with actual seller address
  const amount = "1000000"; // 1 USDC (6 decimals)
  await initiateEscrow(client, account, seller, amount);
  
  // 4. Release funds
  await releaseFunds(client, account);
}

// Approve escrow contract to spend USDC
// async function approveUSDC(client, account) {
//   const approveMsg = {
//     increase_allowance: {
//       spender: ESCROW_CONTRACT_ADDRESS,
//       amount: "1000000", // 1 USDC
//       expires: null
//     }
//   };

//   console.log("Approving USDC spending...");
//   const result = await client.execute(
//     account.address,
//     USDC_CONTRACT_ADDRESS,
//     approveMsg,
//     "auto"
//   );
  
//   console.log("Approval successful. TX hash:", result.transactionHash);
// }

// Initiate escrow contract
async function initiateEscrow(client, account, seller, amount) {

  const funds = [{ denom: "uxion", amount: amount }];
  const escrowMsg = {
    initiate_escrow: {
      buyer: account.address,
      seller: seller,
      amount: funds
    }
  };
   

  console.log("Initiating escrow...");
  const result = await client.execute(
    account.address,
    ESCROW_CONTRACT_ADDRESS,
    escrowMsg,
    undefined,
    "auto"
  );
  
  console.log("Escrow initiated. TX hash:", result.transactionHash);
}

// Release funds to seller
async function releaseFunds(client, account) {
  const releaseMsg = {
    release_funds: {}
  };

  console.log("Releasing funds to seller...");
  const result = await client.execute(
    account.address,
    ESCROW_CONTRACT_ADDRESS,
    releaseMsg,
    "auto"
  );
  
  console.log("Funds released. TX hash:", result.transactionHash);
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});
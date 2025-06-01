// // import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
// // import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
// // import { GasPrice } from "@cosmjs/stargate";
// // import { readFileSync } from "fs";
const { SigningCosmWasmClient } = require("@cosmjs/cosmwasm-stargate");
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { GasPrice } = require("@cosmjs/stargate");
const {  readFileSync } = require("fs");

// import dotenv from "dotenv"
require("dotenv").config();

// dotenv.config()

const rpcEndpoint = "https://rpc.xion-testnet-2.burnt.com:443";
const mnemonic = process.env.MNEMONIC;
const wasmFilePath = "../artifacts/escrow_contract.wasm";

async function main() {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: "xion",
  });

  const [firstAccount] = await wallet.getAccounts();
  console.log("Wallet address:", firstAccount.address);

  const client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    wallet,
    {
      gasPrice: GasPrice.fromString("0.025uxion")
    }
  );

  const wasmCode = readFileSync(wasmFilePath);
  const uploadReceipt = await client.upload(firstAccount.address, wasmCode ,"auto");
  console.log("Upload successful, code ID:", uploadReceipt.codeId);

  const initMsg = {};
  const instantiateReceipt = await client.instantiate(firstAccount.address, uploadReceipt.codeId, initMsg, "Escrow", "auto");
  console.log("Fist account", firstAccount.address);
  console.log("Contract instantiated at:", instantiateReceipt.contractAddress);
}
main().catch(console.error);



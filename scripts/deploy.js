// // // import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
// // // import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
// // // import { GasPrice } from "@cosmjs/stargate";
// // // import { readFileSync } from "fs";
// const { SigningCosmWasmClient } = require("@cosmjs/cosmwasm-stargate");
// const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
// const { GasPrice } = require("@cosmjs/stargate");
// const {  readFileSync } = require("fs");

// // import dotenv from "dotenv"
// require("dotenv").config();

// // dotenv.config()

// const rpcEndpoint = "https://rpc.xion-testnet-2.burnt.com:443";
// const mnemonic = process.env.MNEMONIC;
// const wasmFilePath = "../artifacts/escrow_contract.wasm";

// async function main() {
//   const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
//     prefix: "xion",
//   });

//   const [firstAccount] = await wallet.getAccounts();
//   console.log("Wallet address:", firstAccount.address);

//   const client = await SigningCosmWasmClient.connectWithSigner(
//     rpcEndpoint,
//     wallet,
//     {
//       gasPrice: GasPrice.fromString("0.025uxion")
//     }
//   );

//   const wasmCode = readFileSync(wasmFilePath);
//   const uploadReceipt = await client.upload(firstAccount.address, wasmCode ,"auto");
//   console.log("Upload successful, code ID:", uploadReceipt.codeId);

//   const initMsg = {};
//   const instantiateReceipt = await client.instantiate(firstAccount.address, uploadReceipt.codeId, initMsg, "Escrow", "auto");
//   console.log("Fist account", firstAccount.address);
//   console.log("Contract instantiated at:", instantiateReceipt.contractAddress);
// }
// main().catch(console.error);



const { SigningCosmWasmClient } = require("@cosmjs/cosmwasm-stargate");
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { GasPrice } = require("@cosmjs/stargate");
const fs = require("fs");

// Configuration
const config = {
  rpcEndpoint: "https://rpc.malaga-420.cosmwasm.com:443", // Testnet RPC
  chainId: "malaga-420",
  mnemonic: "your-mnemonic-here", // Replace with your mnemonic
  prefix: "wasm",
  gasPrice: GasPrice.fromString("0.025umlg"),
};

// Contract paths - update these paths to your compiled wasm files
const FACTORY_WASM_PATH = "./artifacts/escrow_factory.wasm";
const ESCROW_WASM_PATH = "./artifacts/escrow.wasm";

async function deployContracts() {
  try {
    console.log("🚀 Starting deployment process...");

    // Create wallet and client
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(config.mnemonic, {
      prefix: config.prefix,
    });
    
    const [firstAccount] = await wallet.getAccounts();
    console.log("📝 Deployer address:", firstAccount.address);

    const client = await SigningCosmWasmClient.connectWithSigner(
      config.rpcEndpoint,
      wallet,
      {
        gasPrice: config.gasPrice,
      }
    );

    // Check balance
    const balance = await client.getBalance(firstAccount.address, "umlg");
    console.log("💰 Balance:", balance);

    // Deploy Escrow Contract
    console.log("\n📦 Deploying Escrow contract...");
    const escrowWasm = fs.readFileSync(ESCROW_WASM_PATH);
    const escrowUploadResult = await client.upload(
      firstAccount.address,
      escrowWasm,
      "auto",
      "Escrow Contract"
    );
    console.log("✅ Escrow contract uploaded. Code ID:", escrowUploadResult.codeId);

    // Deploy Factory Contract
    console.log("\n📦 Deploying Factory contract...");
    const factoryWasm = fs.readFileSync(FACTORY_WASM_PATH);
    const factoryUploadResult = await client.upload(
      firstAccount.address,
      factoryWasm,
      "auto",
      "Escrow Factory Contract"
    );
    console.log("✅ Factory contract uploaded. Code ID:", factoryUploadResult.codeId);

    // Instantiate Factory Contract
    console.log("\n🏗️  Instantiating Factory contract...");
    const factoryInstantiateMsg = {
      owner: firstAccount.address,
    };

    const factoryInstantiateResult = await client.instantiate(
      firstAccount.address,
      factoryUploadResult.codeId,
      factoryInstantiateMsg,
      "Escrow Factory",
      "auto",
      {
        admin: firstAccount.address,
      }
    );

    console.log("✅ Factory contract instantiated at:", factoryInstantiateResult.contractAddress);

    // Update factory contract with escrow code ID
    console.log("\n🔧 Setting escrow code ID in factory...");
    // Note: You'll need to add a SetEscrowCodeId execute message to your factory contract
    // For now, we'll store it in a config file
    
    const deploymentInfo = {
      factoryCodeId: factoryUploadResult.codeId,
      escrowCodeId: escrowUploadResult.codeId,
      factoryAddress: factoryInstantiateResult.contractAddress,
      deployerAddress: firstAccount.address,
      chainId: config.chainId,
      rpcEndpoint: config.rpcEndpoint,
      deployedAt: new Date().toISOString(),
    };

    // Save deployment info
    fs.writeFileSync("deployment-info.json", JSON.stringify(deploymentInfo, null, 2));
    console.log("\n💾 Deployment info saved to deployment-info.json");

    console.log("\n🎉 Deployment completed successfully!");
    console.log("📋 Summary:");
    console.log(`   Factory Code ID: ${factoryUploadResult.codeId}`);
    console.log(`   Escrow Code ID: ${escrowUploadResult.codeId}`);
    console.log(`   Factory Address: ${factoryInstantiateResult.contractAddress}`);

    return deploymentInfo;

  } catch (error) {
    console.error("❌ Deployment failed:", error);
    throw error;
  }
}

// Run deployment
if (require.main === module) {
  deployContracts()
    .then(() => {
      console.log("✅ Deployment script completed");
      process.exit(0);
    })
    .catch((error) => {
      console.error("❌ Deployment script failed:", error);
      process.exit(1);
    });
}

module.exports = { deployContracts };

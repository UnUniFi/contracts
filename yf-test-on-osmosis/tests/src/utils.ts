import { readFileSync } from "fs";

import { AckWithMetadata, CosmWasmSigner, RelayInfo, testutils } from "@confio/relayer";
// import { fromBase64, fromUtf8 } from "@cosmjs/encoding";
// import { assert } from "@cosmjs/utils";

const { fundAccount, generateMnemonic, osmosis: oldOsmo, signingCosmWasmClient } = testutils;
const osmosis = { ...oldOsmo, minFee: "0.025uosmo" };

export async function setupContract(
  cosmwasm: CosmWasmSigner,
  contracts: Record<string, string>
): Promise<Record<string, number>> {
  const results: Record<string, number> = {};

  for (const name in contracts) {
    const path = contracts[name];
    console.info(`Storing ${name} from ${path}...`);
    const wasm = await readFileSync(path);
    const receipt = await cosmwasm.sign.upload(cosmwasm.senderAddress, wasm, "auto", `Upload ${name}`);
    console.debug(`Upload ${name} with CodeID: ${receipt.codeId}`);
    results[name] = receipt.codeId;
  }

  return results;
}

// This creates a client for the CosmWasm chain, that can interact with contracts
export async function setupOsmosisClient(): Promise<CosmWasmSigner> {
  // create apps and fund an account
  const mnemonic = generateMnemonic();
  const osmosisSigner = await signingCosmWasmClient(osmosis, mnemonic);
  await fundAccount(osmosis, osmosisSigner.senderAddress, "4000000");
  return osmosisSigner;
}
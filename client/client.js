import { fromHex, toHex } from "@cosmjs/encoding";
import { sha256, Secp256k1 } from "@cosmjs/crypto";
import { createRequire } from "module";

const require = createRequire(import.meta.url);
const elliptic = require("elliptic");

async function main() {
  const privkey = fromHex(
    "43a9c17ccbb0e767ea29ce1f10813afde5f1e0a7a504e89b4d2cc2b952b8e0b9"
  );

  const keypair = await Secp256k1.makeKeypair(privkey);
  const pubKey = keypair.pubkey;

  const message = "Hello world!";
  const messageHash = new TextEncoder().encode(message);
  const signature = await Secp256k1.createSignature(
    messageHash,
    keypair.privkey
  );

  console.log("SECP256K1_MESSAGE_HEX", toHex(messageHash));
  console.log(
    "SECP256K1_SIGNATURE_HEX",
    toHex(signature.toFixedLength().slice(0, 64))
  );
  console.log("SECP256K1_PUBLIC_KEY_HEX", toHex(pubKey));

  const ok = await Secp256k1.verifySignature(
    signature,
    messageHash,
    keypair.pubkey
  );
  console.log("Signature valid?", ok);
}

main();

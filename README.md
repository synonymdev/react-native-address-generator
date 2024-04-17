# react-native-address-generator

A Bitcoin Address Generator for React Native Projects.

## Installation

```sh
npm install react-native-address-generator
```

## Usage

```js
import { getAddress, getScriptHash, getPrivateKey } from 'react-native-address-generator';

const mnemonic =
  'lazy rally chat way pet outside flame cup oval absurd innocent balcony';
const passphrase = 'passphrase';
const path = "m/84'/1'/0'/0/0";
const network = 'testnet';

const getAddressRes = await getAddress({
  mnemonic,
  path,
  network,
  passphrase,
});
if (getAddressRes.isErr()) {
  console.log(getAddressRes.error.message);
  return;
}
console.log(getAddressRes.value);

const address = getAddressRes.value.address;

const getScriptHashRes = await getScriptHash({
  address,
  network,
});
if (getScriptHashRes.isErr()) {
  console.log(getScriptHashRes.error.message);
  return;
}
console.log(getScriptHashRes.value);

const getPrivateKeyRes = await getPrivateKey({
  mnemonic,
  path,
  network,
  passphrase,
});
if (getPrivateKeyRes.isErr()) {
  console.log(getPrivateKeyRes.error.message);
  return;
}
console.log(getPrivateKeyRes.value);
```

## Update Bindings

After making changes to any of the Rust files, the bindings will need to be updated. To do this, run the following command:

```sh
npm run update-bindings
```

Finally, ensure that `AddressGeneratorModule.kt`, `AddressGenerator.swift`, `AddressGenerator.mm` & `src/index.tsx` are updated accordingly based on the changes made to the Rust files.

## License

MIT

---

## Resources

 - Project created with: [create-react-native-library](https://github.com/callstack/react-native-builder-bob)
 - [Building an Android App with Rust Using UniFFI](https://forgen.tech/en/blog/post/building-an-android-app-with-rust-using-uniffi)
 - [Building an iOS App with Rust Using UniFFI](https://forgen.tech/en/blog/post/building-an-ios-app-with-rust-using-uniffi)


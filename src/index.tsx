import { NativeModules, Platform } from 'react-native';
import { ok, err, type Result } from '@synonymdev/result';

const LINKING_ERROR =
  `The package 'react-native-address-generator' doesn't seem to be linked. Make sure: \n\n` +
  Platform.select({ ios: "- You have run 'pod install'\n", default: '' }) +
  '- You rebuilt the app after installing the package\n' +
  '- You are not using Expo Go\n';

const AddressGenerator = NativeModules.AddressGenerator
  ? NativeModules.AddressGenerator
  : new Proxy(
      {},
      {
        get() {
          throw new Error(LINKING_ERROR);
        },
      }
    );

type TNetwork =
  | 'bitcoin'
  | 'mainnet'
  | 'testnet'
  | 'bitcoinTestnet'
  | 'regtest'
  | 'bitcoinRegtest';

export async function getAddress({
  mnemonic,
  path,
  network,
  passphrase = '',
}: {
  mnemonic: string;
  path: string;
  network: TNetwork;
  passphrase?: string;
}): Promise<
  Result<{
    address: string;
    path: string;
    publicKey: string;
  }>
> {
  const res = await AddressGenerator.getAddress(
    mnemonic,
    path,
    network,
    passphrase
  );
  if (res[0] === 'error') {
    return err(res[1]);
  }
  return ok(res[1]);
}

export async function getScriptHash({
  address,
  network,
}: {
  address: string;
  network: TNetwork;
}): Promise<Result<string>> {
  const res = await AddressGenerator.getScriptHash(address, network);
  if (res[0] === 'error') {
    return err(res[1]);
  }
  return ok(res[1]);
}

export async function getPrivateKey({
  mnemonic,
  path,
  network,
  passphrase = '',
}: {
  mnemonic: string;
  path: string;
  network: TNetwork;
  passphrase?: string;
}): Promise<Result<string>> {
  const res = await AddressGenerator.getPrivateKey(
    mnemonic,
    path,
    network,
    passphrase
  );
  if (res[0] === 'error') {
    return err(res[1]);
  }
  return ok(res[1]);
}

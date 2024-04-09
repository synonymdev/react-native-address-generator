import * as React from 'react';

import { StyleSheet, View, Button } from 'react-native';
import {
  getAddress,
  getPrivateKey,
  getScriptHash,
} from 'react-native-address-generator';

const mnemonic =
  'lazy rally chat way pet outside flame cup oval absurd innocent balcony';
const path = "m/84'/1'/0'/0/0";
const network = 'testnet';
const address = 'tb1q66g7wtgaeazeaze65rmeyavulkhum6mkks54fz';

export default function App() {
  return (
    <View style={styles.container}>
      <Button
        title={'getAddress'}
        onPress={async (): Promise<void> => {
          const res = await getAddress({
            mnemonic,
            path,
            network,
          });
          if (res.isErr()) {
            console.log(res.error.message);
            return;
          }
          console.log(res.value);
        }}
      />
      <Button
        title={'getScriptHash'}
        onPress={async (): Promise<void> => {
          const res = await getScriptHash({
            address,
            network,
          });
          if (res.isErr()) {
            console.log(res.error.message);
            return;
          }
          console.log(res.value);
        }}
      />
      <Button
        title={'getPrivateKey'}
        onPress={async (): Promise<void> => {
          const res = await getPrivateKey({
            mnemonic,
            path,
            network,
          });
          if (res.isErr()) {
            console.log(res.error.message);
            return;
          }
          console.log(res.value);
        }}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  box: {
    width: 60,
    height: 60,
    marginVertical: 20,
  },
});

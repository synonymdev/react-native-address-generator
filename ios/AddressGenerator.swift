import Foundation

@objc(AddressGenerator)
class AddressGenerator: NSObject {

  @objc(getAddress:derivationPath:networkType:bip39Passphrase:withResolver:withRejecter:)
  func getAddress(_ mnemonic: String, derivationPath: String, networkType: String, bip39Passphrase: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) {
      do {
          let result = try react_native_address_generator.getAddress(mnemonic: mnemonic, derivationPath: derivationPath, networkType: networkType, bip39Passphrase: bip39Passphrase)
          resolve(result)
      } catch {
          reject("GetAddressError", "Failed to get address", error)
      }
  }

  @objc(getPrivateKey:derivationPath:networkType:bip39Passphrase:withResolver:withRejecter:)
  func getPrivateKey(_ mnemonic: String, derivationPath: String, networkType: String, bip39Passphrase: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) {
      do {
          let result = try react_native_address_generator.getPrivateKey(mnemonic: mnemonic, derivationPath: derivationPath, networkType: networkType, bip39Passphrase: bip39Passphrase)
          resolve(result)
      } catch {
          reject("GetPrivateKeyError", "Failed to get private key", error)
      }
  }

  @objc(getScriptHash:network:withResolver:withRejecter:)
  func getScriptHash(_ address: String, network: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) {
      do {
          let result = try react_native_address_generator.getScriptHash(address: address, network: network)
          resolve(result)
      } catch {
          reject("GetScriptHashError", "Failed to get script hash", error)
      }
  }
}

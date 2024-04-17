#import <Foundation/Foundation.h>
#import <React/RCTBridgeModule.h>

@interface RCT_EXTERN_MODULE(AddressGenerator, NSObject)

RCT_EXTERN_METHOD(getAddress:(NSString *)mnemonic
                  derivationPath:(NSString *)derivationPath
                  networkType:(NSString *)networkType
                  bip39Passphrase:(NSString *)bip39Passphrase
                  withResolver:(RCTPromiseResolveBlock)resolve
                  withRejecter:(RCTPromiseRejectBlock)reject)

RCT_EXTERN_METHOD(getPrivateKey:(NSString *)mnemonic
                  derivationPath:(NSString *)derivationPath
                  networkType:(NSString *)networkType
                  bip39Passphrase:(NSString *)bip39Passphrase
                  withResolver:(RCTPromiseResolveBlock)resolve
                  withRejecter:(RCTPromiseRejectBlock)reject)

RCT_EXTERN_METHOD(getScriptHash:(NSString *)address
                  network:(NSString *)network
                  withResolver:(RCTPromiseResolveBlock)resolve
                  withRejecter:(RCTPromiseRejectBlock)reject)

+ (BOOL)requiresMainQueueSetup
{
  return NO;
}

@end

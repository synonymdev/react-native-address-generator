package com.addressgenerator

import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.bridge.ReactContextBaseJavaModule
import com.facebook.react.bridge.ReactMethod
import com.facebook.react.bridge.Promise
import com.facebook.react.bridge.Arguments
import uniffi.mobile.getPrivateKey
import uniffi.mobile.getScriptHash
import uniffi.mobile.getAddress

class AddressGeneratorModule(reactContext: ReactApplicationContext) :
  ReactContextBaseJavaModule(reactContext) {

  override fun getName(): String {
    return NAME
  }

  override fun getConstants(): Map<String, Any> {
    return mapOf("requiresMainQueueSetup" to false)
  }

  @ReactMethod
  fun getAddress(mnemonic: String, derivationPath: String, networkType: String, promise: Promise) {
    try {
      val result = getAddress(mnemonic, derivationPath, networkType)
      val array = Arguments.createArray().apply {
        result.forEach { pushString(it) }
      }
      promise.resolve(array)
    } catch (e: Exception) {
      promise.reject("Error", e.message)
    }
  }

  @ReactMethod
  fun getPrivateKey(mnemonic: String, derivationPath: String, networkType: String, bip39_passphrase: String, promise: Promise) {
    try {
      val result = getPrivateKey(mnemonic, derivationPath, networkType, bip39_passphrase)
      val array = Arguments.createArray().apply {
        result.forEach { pushString(it) }
      }
      promise.resolve(array)
    } catch (e: Exception) {
      promise.reject("Error", e.message)
    }
  }

  @ReactMethod
  fun getScriptHash(address: String, networkType: String, promise: Promise) {
    try {
      val result = getScriptHash(address, networkType)
      val array = Arguments.createArray().apply {
        result.forEach { pushString(it) }
      }
      promise.resolve(array)
    } catch (e: Exception) {
      promise.reject("Error", e.message)
    }
  }

  companion object {
    const val NAME = "AddressGenerator"
  }
}

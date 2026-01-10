# Keyring Demo

This directory contains a Tauri v2.0 cross-platform app that serves two purposes:

- It provides sample code for developers on how to integrate the keyring ecosystem into their apps.
- It allows users and developers both with the ability to poke around in the keyring-compatible stores used by their apps.

The documentation for using the app is on the Keyring ecosystem wiki. This document provides instructions for how to install and to build the app for various platforms.

_Everything below this line is in progress!!_
***

## Installation Instructions

Various platforms can use apps distributed through various channels. This list is organized by platform.

### macOS

You will be able to get Keyring Demo through the Mac App Store once it’s approved. For now, you can get the public beta by request to @brotskydotcom on GitHub. The version that’s available from the App Store is a sandboxed app, so it has access to both to Keychain Services credentials and to Protected Data credentials.

You will be able to get Keyring Demo through 

Note: All distributed versions on macOS run only on Apple Silicon machines. It’s possible to build the app for the older x86 machines, but because those machines don’t have protected data you will only have access to the Keychain.

## Build Instructions

Various platforms can use apps distributed through various channels. This list is organized by distribution channel.

### Apple App Store

In order to distribute through the Apple App Store, you must be an Apple Developer. The Tauri 2.0 documentation has tons of information about how developers configure their builds so they are signed correctly for App Store upload. The configuration files in this repo have all been configured correctly for the `brotskydotcom` App Store developer, team ID `85H73V9R3F`. In order for you to publish this app under your own team ID, you will need to alter all the configuration files to use a bundle ID that *you* have registered under your own team ID, and to refer to your own signing certificates and so on.

To build the iOS app for the app store, follow these steps:

1. Edit the `src-tauri/tauri.conf.json` file and change the `iOS > bundleVersion` parameter to be `1.0` (removing the last digit), but remember the last digit and increment it by one (let’s say it was 7 and so it becomes 8).

2. Give this build command:
   ```shell
   npm run tauri ios build -- --build-number 8 --open
   ```

   (Notice the 8 that you calculated in the first step gets used in this step.) This ensures that the build uploaded to the App Store has a bigger build number (technically a higher _CFBundleVersion_) than prior builds, which is a requirement.

3. Now take the 8 you calculated in the first step and save it as the last digit of the bundle version (so it becomes, say,  `1.0.8` rather than the `1.0.7` it started off as). That way you can remember the last built version for next time.

To build the macOS app for the app store, follow these steps (after having fixed all the configuration files):

1. Edit the `src-tauri/tauri.conf.json` file and change the `macOS > bundleVersion` parameter to be one patch version greater than it was. So, for example, if it was `1.0.4`, you would change it to `1.0.5`. This ensures that the build number is higher than the one last uploaded to the app store, which is a requirement.

2. Give this sequence of commands, replacing `85H73V9R3F` with your Team ID, `SSD3DPQ9MU` with your AppStoreConnect API Key ID, and `69a6de7e-5cea-47e3-e053-5b8c7c11a4d1` with your AppStoreConnect API Issuer ID:
   ```shell
   npm run tauri build -- --no-bundle
   npm run tauri bundle -- --bundles app --config src-tauri/tauri.appstore.conf.json
   pushd src-tauri/target/release/bundle/macos
   xcrun productbuild --sign "85H73V9R3F" --component "keyring-demo.app" /Applications "keyring-demo.pkg"
   xcrun altool --upload-app --type macos --file "keyring-demo.pkg" --apiKey SSD3DPQ9MU --apiIssuer 69a6de7e-5cea-47e3-e053-5b8c7c11a4d1
   ```

### Google Play

In order to distribute Android apps through Google Play you must be a Google Play Developer. The Tauri 2.0 documentation has tons of information about how developers configure their builds so they are signed correctly for Google Play upload. The configuration files in this repo have all been configured correctly for the `brotskydotcom` Google Play developer, org ID `85H73V9R3F`.

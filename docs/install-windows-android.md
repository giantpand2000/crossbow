# Android setup on Windows

## Install necessary packages

1. Use crossbundle install command or download and install [Android Studio](https://developer.android.com/studio).
2. Start Android Studio, and go through the `Android Studio Setup Wizard` with the `Custom` option and install the following (or install them in `SDK Manager`):
   - Android SDK
   - NDK (Side by side)
   - Android SDK Command-line Tools
   - Android SDK Build-Tools
   - Android SDK Platform-tools

## Add environment variables

Take these steps to add Android-related environment variables:

- From the Start search bar, enter ‘env’ and select **Edit environment variables for your account**.
- Add `ANDROID_SDK_ROOT`|| `ANDROID_SDK_PATH` || `ANDROID_HOME` variable with value `<path_to_sdk>\sdk\ndk\<version>`.<br/>(ex. `C:\Users\username\AppData\Local\Android\Sdk\ndk\23.1.7779620`)
- Add `ANDROID_SDK_ROOT` variable with value `<path_to_sdk>\sdk`.<br/>(ex. `C:\Users\username\AppData\Local\android\sdk`)

If u will build application with emulator u should add this environment variables:

- Add `<path_to_sdk>\sdk\tools\bin` to `PATH` variable.
- Add `<path_to_sdk>\sdk\emulator` to `PATH` variable.

Also, we need to make sure we have a [java runtime environment](https://www.oracle.com/java/technologies/downloads/) (JRE) installed. We need a key tool utility from there. <br/>
To make sure it's present type this command: `keytool -h`

- If command above fails, add `<path_to_jre>\bin` to `PATH` environment variable.<br/>(ex. `C:\Program Files\Android\Android Studio\jre\bin`)

You have to close and reopen any existing console windows for these changes to take effect.

### If you want to generate AAB (Android App Bundle) u will need to install Bundletool.

If you haven't already done so, download bundletool from the [`GitHub repository`](https://github.com/google/bundletool/releases).

- Add `BUNDLETOOL_PATH` variable with value `<path_to_bundletool>`.

## Set up your Android device

To prepare to run your `Crossbow` app on an Android device, you need an Android device running Android 4.1 (API level 16) or higher.

1. Enable **Developer options** and **USB debugging** on your device. Detailed instructions are available in the [Android documentation](https://developer.android.com/studio/debug/dev-options).
2. Windows-only: Install the [Google USB Driver](https://developer.android.com/studio/run/win-usb).
3. Using a USB cable, plug your phone into your computer. If prompted on your device, authorize your computer to access your device.

## Set up the Android emulator

To prepare to run and test your Flutter app on the Android emulator, follow these steps:

1. Enable [`VM acceleration`](https://developer.android.com/studio/run/emulator-acceleration) on your machine.
2. Launch **Android Studio**, click the **AVD Manager** icon, and select **Create Virtual Device**.
3. Choose a device definition and select **Next**.
4. Select one or more system images for the Android versions you want to emulate, and select **Next**. An x86 or x86_64 image is recommended.
5. Under Emulated Performance, select **Hardware - GLES 2.0** to enable [`hardware acceleration`](https://developer.android.com/studio/run/emulator-acceleration).
6. Verify the AVD configuration is correct, and select **Finish**. (For details on the above steps, see [`Managing AVDs`](https://developer.android.com/studio/run/managing-avds))
7. In Android Virtual Device Manager, click Run in the toolbar. The emulator starts up and displays the default canvas for your selected OS version and device.

## Install necessary rustup targets

Run the following command:

```sh
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android
```

## Next step

[`Hello World! application`](https://github.com/dodorare/crossbow/wiki/Hello-World!) with Crossbow


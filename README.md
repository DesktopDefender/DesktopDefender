
# Desktop Defender Setup Guide

Welcome to the setup guide for Desktop Defender. Follow the steps below to configure and start using the application.

![alt text](image.png)


## Install
Desktop Defender can be installed using one of the following two methods described below.

### 1. Downloading a binary

To install the Desktop Defender binary, please visit the [Desktop Defender Landing Page](https://desktopdefender.app) and download the application. Currently, the app only supports Apple Silicon devices (Mac M1-M3) as it hasn't been tested on other devices.

#### Configuring macOS Gatekeeper

Due to the lack of digital signature on the binary from Apple, the macOS Gatekeeper will quarantine the downloaded binary. To remove this quarantine attribute and proceed with the installation, perform the following steps:

1. Open a Terminal application.

2. Execute the command below to clear the quarantine attributes, allowing the app to run smoothly:

   ```bash
   xattr -cr /Applications/Desktop\ Defender.app
   ```

#### Opening the application

Now, Desktop Defender should be available on your device. To open it, you can open your applications folder or through [spotlight search](https://support.apple.com/en-is/guide/mac-help/mchlp1008/mac).

### 2. Running the source code

Running the app through source code is generally more complex, but is both safer than downloading an unsigned binary from the internet and is more flexible, especially for non Apple Silicon devices.

#### Prerequisites

To be able to download and run the source code, you must ensure that you have these dependencies on your device:

* [`rust >= v1.60`](https://www.rust-lang.org/)
    - To verify you have these installed, you can run:
    * `rustc --version`
    * `cargo --version`
* [`node >= v18.17 & npm >= v9.6`](https://nodejs.org/en)
    - To verify you have these installed, you can run:
    * `node --version`
    * `npm --version`

#### Configuration

Running the source code requires a single environment variable, `IPINFO_TOKEN`, which is used to access [IPinfo's](https://ipinfo.io) API for mapping ip's to countries. 
Currently, instead of fetching the environment variable from a `.env` file, the code fetches it from the actual environment. The steps below show how to set this token into your environment.


1. **Acquire a token**:
    Either register at [IPinfo for Developers](https://ipinfo.io/developers) to get your own API token or ask us for a token.

2. **Set Environment Variable**:
   Store your API token as an environment variable to be used by the application. Open your terminal and run the following commands:
   ```bash
   export IPINFO_TOKEN="your_token_here"
   echo 'export IPINFO_TOKEN="your_token_here"' >> ~/.zshrc
   source ~/.zshrc
   ```


#### Setup

To set up the code, you must follow the steps below:

1. **Clone the Repository**:
    Clone the Desktop Defender repository to your local machine using the following command in a terminal:
    ```bash
    git clone https://github.com/DesktopDefender/DesktopDefender.git
    ```

2. **Install Dependencies**:
    In the terminal, change to the project directory and install the required Node modules:
    ```bash
    cd DesktopDefender
    npm install
    ```
    This could take a while.

#### Running the code

To run the code in development mode, you can run:
```bash
npm run tauri dev
```

This will compile the rust backend and run the frontend on port 3000. Make sure you don't have any projects running on port 3000 yourself before running this command.

Once the script has finished compiling, the project should be live on your device.


## Building for Production
To build the application for production, which compiles the application and adds to your machine:

```bash
npm run tauri build
```

With no flags, this command will build an optimized bundle specifiacally for the OS + architecture combination your device is running.

To build for other platforms using the `--target` flag, refer to [Tauri documentation](https://tauri.app/v1/guides/building/).



## Persistence

### Data Storage

When building Desktop Defender, a `.dd/` directory containing SQLite databases is created within your home directory. This directory is used to store application data persistently.

### Uninstalling

If you decide to uninstall Desktop Defender, remember to manually remove the `.dd/` directory to clean up all stored data. This step ensures that no residual data remains on your system. However, nothing bad happens if you do not delete it.



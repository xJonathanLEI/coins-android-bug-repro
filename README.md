# coins-android-bug-repro

Reproducer for [a bug](https://github.com/summa-tx/coins/pull/129#issuecomment-1835245612) introduced in [summa-tx/coins#129](https://github.com/summa-tx/coins/pull/129). This reproducer is supposed to be run in [Termux](https://termux.dev/) on an Android device.

## Running

Install [Termux](https://github.com/termux/termux-app) and [Termux API](https://github.com/termux/termux-api) (_NOT_ from Google Play store). Inside Termux, update package registry index:

```console
pkg update
```

Then install necessary dependencies:

```console
pkg install git rust libusb termux-api
```

Then clone and change directory into this repository.

Now, connect your Ledger device, unlock and enter the Ethereum app. First, use this command to determine the full device path:

```console
termux-usb -l
```

which shows a list of paths to the connected devices. There should only be one entry (unless you have other devices connected). An example output looks like this:

```log
[
  "/dev/bus/usb/002/003"
]
```

Now, use this value to run the following command. Replace the path with the value you got from the command above:

```console
termux-usb -r -E -e "cargo run" /dev/bus/usb/002/003
```

which should ask for a signature for send a `0 ETH` transaction to the zero address from your Ledger device. This is using the `main` branch for `coins-ledger`.

Now, change the Git `rev` used for the `coins-ledger` dependency to point to the PR head. Alternatively, use the `Cargo.pr.toml` file, which points to commit `1d578d4`

```console
cp Cargo.pr.toml Cargo.toml
```

And run the `cargo run` command again:

```console
termux-usb -r -E -e "cargo run" /dev/bus/usb/002/003
```

This time, the application should fail.

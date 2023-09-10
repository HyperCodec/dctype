# dctype
Tool to infinitely type in a set of discord channels.

### Disclaimer
While I have not seen this flag any sort of ratelimit/ban, be aware that it is a possibility. Use at your own risk.

### How to install
To install this tool, first ensure that you have the latest Rust/Cargo installed. Then download this repo to a folder, open commandline in that folder, and run `cargo install --path .`. Restart/refresh the terminal before you begin using the tool.

Alternatively, you can run `cargo build --release` in the folder, then navigate to `target/release` and use that compiled executable if you don't want to entirely install this tool.

### How to use
The command syntax is `dctype --channel-list "somefile.txt" --token "discordacountttoken"`
(or alternative formatting: `dctype -c "somefile.txt" -t "discordaccounttoken"`)

You can get your token via a number of methods, simply search the web for how to do so.

The channel list txt file should be formatted with each channel ID on a newline, like this:
```
000000000000000000
111111111111111111
222222222222222222
333333333333333333
```

Make sure the botted account has access to the channel(s) that you type in or else it will return `403 Forbidden` (and possibly get the account banned if left running long enough)
# Tweet like

This mini command line app is for automate the process of like to a tweet, but with the advantage that be can filter by a blacklist of words that you dont not want in the tweet.

> Warning
> 
> - This project has been with the learning motivation
> - You will that can find code that is not well implemented
> 
> For this reason, you can leave a feedback in the issues, this it will be very helpful for me.
 

## Requirements

- Should have installed rust in your os, to more info go [here](https://www.rust-lang.org/tools/install).
- Should have installed the following target `x86_64-pc-windows-gnu`, for that, with this following command `rustup toolchain install stable-x86_64-pc-windows-gnu` you can.
- In development you need a twitter app and add the tokens in of the `.env` file, this explined more ahead.
- Twitter account to can at the use of the app.

## How configure the `.env` file

This file is to load a environment variables, for this be use a dotenv crate in the `build.rs` file. This file is to load a environment variables, for this be use a dotenv crate in the `build.rs` file. In this file you will add the tokens of you  twitter app.

```shell
TWITTER_CONSUMER_KEY=...
TWITTER_CONSUMER_SECRET=...
```

When you will compiled proyect this file is used to insert this credentials to the code, this only is on development, to other occasions use the environments variables.

Can see this in the `./build.rs` file.

## How to compile in development

To compile app sun has to axecute this cargo command

```shell
$ cargo build
```

## How to run in development

To run the app in you development process should use this cargo command

```shell
$ cargo run
```
> This command also does a simple build of the app

## Flow to generate a executable file to mac and windows

### Mac

1. Create a release with the following cargo command `cargo build --release`, this will create the execute file in `target/release/tweet-like`.
2. To execute the file use this  `./target/release/tweet-like` .

### Windows

1. Create a release with the following cargo command `cargo build --release --target x86_64-pc-windows-gnu` this will create the execute file with extension `.exe` in `target/x86_64-pc-windows-gnu/release/tweet-like.exe`.
2. To execute this file you must have a machine whit windows os installed and do double click in the generated file that it's in this route `target/x86_64-pc-windows-gnu/release/tweet-like.exe`.

After pf the first execution the program, you will should set a settings to the search and the filter, this should you on the `settings.yaml` file. Here an example of a configuration:

```yaml
# This field indicate the number of results will.
searchCount: 25
# This field is a list of the terminus will searching on twitter.
searchTerms:
  - #rustlang
# This field is a black list of terminus will use for discard tweets.
blackList:
  - java
```

When you execute the program again, the search will be carried out with this configuration.

What's more of the `settings.yaml` file, be will create an `storage.ron` ([https://github.com/ron-rs/ron](https://github.com/ron-rs/ron)) file that is a database file, what contain the credentials of twitter user and the search results with the status before of the filtered.

## What missing?

- [ ] Testing
- [ ] Generating the documentation in the files.
- [ ] Bench

---

If you like this and do you want support me, you will be can here

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/V7V43U127)
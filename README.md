# Cosmo Text Editor 

Cosmo is a terminal-based text editor like vim or emacs I made using Ratatui. It's... not very good
to be honest (I heavily advise against using it for any semi-serious application). I am mainly
creating this project so I can learn more about creating text-based user interfaces in preparation
for another project I have coming up.

## Build/Run

You need to have `cargo` installed on your system. Run `cargo build --locked --release`, then run
`cargo r --release -- {PATH_TO_FILE}` where `PATH_TO_FILE` is the path of the file you want to open
or create. You can also copy the executable out of the `target/release/build/` directory.

## Demo
You can see a demo of Cosmo [here](https://youtu.be/sLS0zSbOcWU).

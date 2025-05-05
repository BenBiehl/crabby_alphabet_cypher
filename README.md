# Mono-alphabetic substitution cypher implementation using eframe template

## Created by Benjamin Biehl

### Testing and Building application

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

Running the command will automatically open the application, or you can locate the .exe in target/release/ to run the application after build.

### How to use

Once you run the application, you will see four tabs: Quit, About, Encrypt, and Decrypt.

Quit will just simply close to the program, same behavior as hitting the "x" button.

About is a brief description on what the program does.

Encrypt will allow you to generate random keys for encryption, you can also paste keys into the key text box to test if they're valid.

Decrypt will allow you to decypt messages using the generated/pasted key. The key state is mainted during tab switching, so don't worry about constantly having to repaste the key.


# Install Rust and Cargo

The easiest way to get Cargo is to install the current stable release of Rust by using rustup. Installing Rust using rustup will also install Cargo.

On Linux and macOS systems, this is done as follows:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

It will download a script and start the installation. If everything goes well, you’ll see this appear:

```
Rust is installed now. Great!
```

On Windows, download and run `rustup-init.exe`. It will start the installation in a console and present the above message on success.

For other installation options and information, visit the [install page](https://www.rust-lang.org/tools/install) of the Rust website.



---

# Running the Program

To run the program, simply use:

```sh
cargo run
```

## **Managing `scanned.csv`**

- The `scanned.csv` file contains IDs of all students who have scanned.
- This file is located at the **root level of the project**.
- If you need to **reset the list**, simply **delete the file**.
- To **import from another device**, paste the `scanned.csv` file at the root level of the project.
- If the file does **not exist**, the application will **create an empty csv automatically** when run.
- If the file **exists but is empty**, the program will **add the required headers**.

As an end user, just run:

```sh
cargo run
```

unless you want to reset or import the `scanned.csv` file.

# **Exiting the Program**

- To exit the program, press **CTRL + C**.
- All saves happen **immediately** at the time of receiving input, so this is a completely safe way to exit.
- There is **no other way** to exit the program.
- Running the program again **continues with previously entered data** and does not cause any issues.

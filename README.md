# Icome Calculator

A simple app to calculate monthly expenses and taxes. Started as a project idea from the good input of [Travis Media](https://www.youtube.com/watch?v=7aFgeUG9TK4&list=PL22QEtzSG4EUDm9sVwqoOHCgWV4tTYfvk&index=63)

## About

This app is a Rust application with Slint as toolkit
for the user interface. It demonstrates the integration between the `.slint` UI markup and
Rust code, how to trigger react to callbacks, get and set properties and use basic widgets.

## Usage

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.
2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
    ```
    cargo install cargo-generate
    ```
3. Set up a sample project with this template
    ```
    cargo generate --git https://github.com/slint-ui/slint-rust-template --name my-project
    cd my-project
    ```
3. Build with cargo
    ```
    cargo build
    ```
4. Run the application binary
     ```
     cargo run
     ```

We recommend using an IDE for development, along with our [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).


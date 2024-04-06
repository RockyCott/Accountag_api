# Accountag's API 📱

<p align="center">
  <a href="https://github.com/RockyCott/Accountag" target="blank"><img src="./assets/logo.jpg" width="120" alt="Nest Logo" style="border-radius: 15px" /></a>
</p>
<p align="center">
API backend diseñada para gestionar y almacenar la información que se manipula en la app Accountag.
<p align="center">

## Tecnologías Utilizadas en la API 🛠️

* [Rust [v1.74.1]](https://www.rust-lang.org): programming language that focuses on reliability and stability.

* [axum](https://crates.io/crates/axum): web framework that focuses on ergonomics and modularity.


* [tokio](https://tokio.rs): platform for writing asynchronous I/O backed applications.

* [cargo-watch](https://crates.io/crates/cargo-watch): utility for automatically restarting the server when file changes are detected.

* [cargo-make](https://crates.io/crates/cargo-make): utility for defining and running custom workflows.

* [Serde](https://crates.io/crates/serde): serialization/deserialization framework.


## Dependencias

Asegúrate de tener [Rust y Cargo](https://www.rust-lang.org/) instalados en tu sistema.

Puedes instalar las dependencias adicionales ejecutando los siguientes comandos:

```bash
# cargo-watch

# Instalar
cargo install cargo-watch
# example of use
cargo watch -x run

# cargo-make

# Instalar
cargo install --force cargo-make
# example of use. see Makefile.toml
cargo make watch
```


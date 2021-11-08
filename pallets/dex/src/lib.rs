[package]
name = "pallet-exchange"
version = "0.1.0"
authors = ["Web3Games Developers"]
edition = "2018"

[dependencies]
codec = { package = "parity-scale-codec", version = "2.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "1.0", default-features = false, features = ["derive"] }

log = { version = "0.4.14", default-features = false }
integer-sqrt = "0.1.2"
frame-support = { git = "https://github.com/paritytech/substrate", default-features = false }
frame-system = { git = "https://github.com/paritytech/substrate", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", default-features = false }
sp-io = { git = "https://github.com/paritytech/substrate", default-features = false }
sp-runtime = { git = "https://github.com/paritytech/substrate", default-features = false }
pallet-timestamp = { git = "https://github.com/paritytech/substrate", default-features = false }
sp-core = { git = "https://github.com/paritytech/substrate", default-features = false }

primitives = { package = "web3games-primitives", path = "../../primitives", default-features = false }
pallet-token-fungible = { path = "../token-fungible", default-features = false }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "log/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "sp-io/std",
    "sp-runtime/std",
    "pallet-timestamp/std",
    "sp-core/std",
    "primitives/std",
    "pallet-token-fungible/std",
]

# Simple yew app to combine twox_128 hashes
### Because I keep getting annoyed when i have to compute them on the cli
##### (and also because yew is cool)


Hash on the web using the same hash functions that the Substrate client/runtime/whatever uses


# To Run:

Install Trunk

`cargo install trunk wasm-bindgen-cli`

Add `wasm32-unknown-unknown` target

`rustup target add wasm32-unknown-unknown`

Run with:

`trunk serve`

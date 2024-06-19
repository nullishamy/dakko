# dakko

a Fediverse client that aims to be more natively integrated than alternatives.

## installation

currently, the only provided installation solution is through Nix.

if you want to build it yourself, you'll need [the tauri pre-requisites](https://tauri.app/v1/guides/getting-started/prerequisites).
you can then run `yarn run tauri build` to build the project. you will find the binary at `./src-tauri/target/debug/dakko`.


## developing

bootstrap the JS system with `yarn install` (or any other JS manager), then run `yarn run tauri dev` to launch the dev server.
this will compile the frontend and the backend, which may take some time.

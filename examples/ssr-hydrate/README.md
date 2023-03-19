Example of SSR and hydrate integration using Axum
================================================================================

The example project shows how the SSR rendering can be incorporated in backend
service and how does it need to be configured to serve built WASM module and
hydrate any SSR rendered components.

Usage
--------------------------------------------------------------------------------

First, build the WASM application using `trunk`. Make sure to enable `hydrate`
feature:

```bash
$ cd app
$ trunk build --features hydrate
```

Second, build and run the backend service. Make sure to run the service from
its project directory. The path to built WASM module is hard-coded, relative
path to it:

```bash
$ cd svc
$ cargo run
```

Open the `127.0.0.1:3000` to access SSR rendered and hydrated application.


Architecture Overview
--------------------------------------------------------------------------------

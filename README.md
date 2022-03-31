<div align="center">

  <h1><code>wasm-pack-template</code></h1>

<strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

-   [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
    between WebAssembly and JavaScript.
-   [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
    for logging panic messages to the developer console.
-   [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
    for small code size.


## Building For Local Dev
* make sure to follow the [sapio install instructions](https://learn.sapio-lang.org/ch01-01-installation.html)

```shell
$  rustup target add wasm32-unknown-unknown
$ export CC=/usr/local/Cellar/llvm/13.0.1/bin/clang
$ export PATH="/usr/local/Cellar/llvm/bin:$PATH"
$ export AR=/usr/local/Cellar/llvm/13.0.1/bin/llvm-ar
$ export CC=/usr/local/Cellar/llvm/13.0.1/bin/clang
```

#### For running the playground
* Run `wasm-pack build` inside your project dictionary
* Run `npm install` inside www folder
* Again run `npm install` inside www folder (just to be sure)
* Finally run `npm run start` inside www and visit http://localhost:8080 to see the results


## For deployment :
* Create a new branch by the name gh-pages
* Github pages should be enabled by default but if not go to Settings -> GitHub Pages and enable it on your gh-pages branch. You will also find the link to your to-be hosted page there
* Make a personal access token (only the token no need for command line here)
* Next we will need to put this token into our travis settings, go to more options -> settings -> Environment Variables and enter the token value (the generated token code) and name as GITHUB_TOKEN, it should look like :
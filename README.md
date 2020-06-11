# Yew Fullstack Boilerplate

This boilerplate was created in order to have a place for sharing efforts in creating full stack applications using Rust. It is supposed to give creators of rust frameworks a notion of how people use their technologies.

It has been created to easily get up to speed with projects that might profit from full stack development in Rust. So why go through all the trouble?

- Modern web applications are bloated, large and slow. Rust and WASM can help to reverse that trend and to build way more capable web applications.
- Rust is fast. The libraries used in this project are fast.
- Rust is safe. At least it's way safer than other languages. Hint: borrow checker.
- Especially large applications in JS or Typescript take more effort to maintain the larger they get. With Rust your application is way less likely to crash once it has been built. At least if you stuck to a few paradigms.

If you want to support my work please consider donating at [Patreon](https://www.patreon.com/lukaswagner).

## Prerequisites

The following software should be installed on your development machine:

- Rust, of course, alongside your favourite IDE (or not if you really like to suffer)
- The latest version of [Node.js](https://nodejs.org/en/download/) ([Deno](https://deno.land/) is not yet tested with this project.)
- The latest version of [Yarn](https://yarnpkg.com/) (installable via `npm install -g yarn` after you installed Node.js)
- The latest version of the [Docker engine](https://www.docker.com/)
- The latest version of [docker-compose](https://docs.docker.com/compose/) (on MacOS it is shipped with the Docker Engine, for usage on Linux please consult the [installation manual](https://docs.docker.com/compose/install/))

## Usage

First thing to do when using this boilerplate for your new project: Update the contents of `backend/src/secret.key`. It needs to be exactly 32 bytes in size, it can be binary.

This boilerplate is built around the concept of single command building. So here are a few commands that will bring you up to speed fast:

- `scripts/run-dev.sh` boots up the whole stack in one go. It will take a little while, especially the first time you run it. _It is the command you are likely to be wanting to use first._
- `scripts/stop-dev.sh` is the command that removes all dev docker containers.
- `scripts/build.sh` will build a production version of the whole stack in one docker container
- `scripts/run.sh` will start up the container built using the previous command.
- `scripts/build-and-run.sh` will do what the previous two commands did but in one go.
- `scripts/stop.sh` will remove all docker containers associated with the production build of your application.
- `scripts/run-dev-force-recreate.sh` will help you if you screwed anything up with `scripts/run-dev.sh`. After it ran successfully you should be able to use `scripts/run-dev.sh` again.

## About

This project is a highly opinionated boilerplate for creating full stack applications with Rust using the following technologies:

- [Yew](https://yew.rs/docs/) and [CSSinRust](https://crates.io/crates/css-in-rust) for the web client
- [Actix web](https://crates.io/crates/actix-web) for the backend
- [MongoDB](https://www.mongodb.com/) for the database

Its authentication mechanism is built around JWT.

Feel free to clone this project and build on top anything you like. If there are updates to the template you can just merge it into your project and you are up to date.

For custimizing the name of the project use your favorite search and replace tool and search for the the terms `yew-fullstack`, `YEW_FULLSTACK`, `Yew Fullstack Boilerplate` and `Yew Fullstack` (in that order) and replace them with whatever you like.

## Showcase

Projects built on top of this boilerplate will have names and links put here. Just tell me or open a PR and I will put them here.

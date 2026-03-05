# Rustceptor

> A simple mock API written in Rust and built for security purposes: phone home, send payloads, etc.

## Boring stuff

### Motivation 

I wanted to learn Rust, and was upset with RequestBin now requiring an account. So instead of looking for an open-source alternative, I decided to build my own.

### Warnings

Deploy and use this at your own risk: if there is a massive vulnerability in this code that allows a kid to steal and leak your 100GB "Homework" folder on some weird forums, that's none of my business.

For now, there is no CSRF protection. I might implement one someday, or not.

This app started as (and still is today) a learning project. That means that you may see some weird things, very unoptimised patterns, and horrible implementations. Developer discretion is advised.

### License

This project is- nah no one cares about that. Just be kind and credit me :)

## Features

- Add, delete and configure route dynamically, with arbitrary response parameters.
- Inspect in detail received requests.

## Installation

### Dependencies

Of course, you first need Rust and Cargo. [Follow the official guide](https://doc.rust-lang.org/book/ch01-01-installation.html) if you don't have it installed yet.

You also need to have a local copy of this repository (yeah, surprisingly).

Yew also requires `trunk` and a WASM compiler to work. Following [the official guide](https://yew.rs/docs/getting-started/introduction) is recommended, though it can be summed up by those two commands (I don't remember if you need privilege for those though):

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

### Run

Once you have all the dependencies, you can first compile the frontend:

```bash
(cd frontend && trunk build)
```

Once successfully built, you can run the server:

```bash
(cd backend && cargo run)
```

This allows the backend to receive everything, and distribute the frontend when requested. By default, the app is available at `http://localhost:8000/front/home`.

### Frontend development

If you are working on the frontend, the trunk hot-reload feature might be useful. Guess what: you can actually access it! You first need to run the backend:

```bash
(cd backend && cargo run)
```

You can then use the *serve* feature to start a frontend server:

```bash
(cd frontend && trunk serve)
```

By default, the frontend server is available at `http://localhost:8080/front/home`.

## Usage

Once logged-in (the master key is defined in `backend/src/main.rs`: `changeme` by default), the user is free to define dynamic routes, or inspect received requests.The backend captures and dispatches every request it receives according to the dynamically defined routes, except for paths:
 - `/front/*`, where the frontend is mounted ;
 - `/backapi/*`, which holds the routes to interact with the backend are defined. 

With two special cases:
 - `/front` redirects to `/front/home` ;
 - `/backapi/capture` is dynamically dispatched, as if it weren't a subpath of `/backapi/*`

The second case allows the backend to capture a request if the frontend server is upfront, as `trunk serve` will proxy `/backapi/*` to `localhost:8000`. That could be useful for having trunk hot reload and test reaction when a captured request event is received. 

## Dev corner

### Tech stack

- Backend: [Rocket 🚀](https://rocket.rs) (Rust)
- Frontend: [Yew](https://yew.rs) (Rust), with [tailwind](https://tailwindcss.com) and [tailwind-animate](https://github.com/jamiebuilds/tailwindcss-animate) (CSS).

The frontend design and base implementation was mainly vibecoded (sorry for all the purists, idc). Lovable made the design, and Gemini translated the JSX / Vite code into Yew.

### Project Structure and development

There are actually two Rust project: one for the backend (in the `/backend` folder), and one for the frontend (yup, in the `/frontend` folder, you guessed it).

They both proxy each other, so, depending on what you wanna work on, you can reach the frontend (to have access to trunk hot reload) or the backend (normal behaviour, requires a compiled frontend). See the [Usage section](#usage).

## TODOs

That's where I store the ideas of features and the things I wanna work on. There is no particular order, and no guarantee that any of those points will ever be actually be checked out.

### Short- and medium-term TODOs

> The next things I wanna work on. I don't have any particular deadline, but those are the next steps.

- ✨ Code review ✨

- Features
    - Mutable default route
    - Captured requests delete
    - Confirm popup on delete request / route
    - Copy on double click for each line in the inspect page
    - Failed auth state(s) & error messages.
    - Make the star route a default one, not a all one
        - Block its path definition
        - Refine that process anyway, quite a dirty way of working for now lol

- UI/UX updates
    - a way to inform the user that a request was received on a given route
    - Design a proper login page

- Fix UI bugs
    - inspect page, request selector, the very top one has a wrong margin with top navbar
    - Switch button : color isn't correctly aligned
    - Switch to "define" when clicking "add route" ; even prevent "inspect"
    - use lucide-yew instead of AI generated SVGs

- Other bugs
    - Redirect LoginPage to HomePage if user already logged-in

- Deployment
    - Provide a nix flake to ease deployment
    - Provide minimal configuration.nix and/or nginx config to have a working setup.
    - Add a Deployment guide
    - Make nginx / proxy use optional (ie be able to add /front/ dispatcher route).

- Codebase changes
    - Change some defaults: Inspect panel by default, maybe select default route by default?
    - Define constants and replace hardcoded values.

### Long-term TODOs, ideas

> Some ideas I had and that would be nice to implement. I don't know if I will actually implement those things, but hey, at least they lay here.

- Use env variables for configuration variables (master key, proxy usage, etc).

- Settings panel (for now, unused)
    - Add some settings: dark theme, tutorial, "log" (actually, toast) levels, etc
    - Add some animation on the settings panel

- RouteDefinition features
    - File management: Be able to send files
    - Redirection, Proxying (be able to fetch something elsewhere and send back that content to the client)
    - Additional random delay for a more realistic API.
    - Wrong / Custom Content-Length.
    - None vs empty Content-Type (for now, only empty).

- Add some presets in Content-Type.

- Support regex in routes.

- Dynamic content ; replace pattern by regex group in body, dynamic response 

- Real login with Multiuser management

- Logs systems for both back and front

- Implement CSRF tokens
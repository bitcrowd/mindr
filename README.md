# Development

## Prerequisites

- Install [rust](https://rust-lang.org/tools/install/)
- Install [dioxus](https://dioxuslabs.com/learn/0.7/getting_started/#install-the-dioxus-cli)
- Install PostgreSql

## Database setup

Open a `psql` shell and type:

```
CREATE DATABASE mindr_dev;
```


## Run client in web

``` bash
cd client
dx serve --platform web
```

## Run client in desktop

``` bash
cd client
dx serve --platform desktop --features desktop
```
or simply

``` bash
cd client
dx serve
```
## Run the server

```
cd server
cargo run
```

## Roadmap

- Server channels and client UI url selection
  - fallback to local state
- Estimations & progress rollup
- Disable/Enable branches
- Show/Hide branches
- Postgres persistance on the server
- Better interactions
  - Reorder nodes
  - Side indicator correctness
- Optional local save & load
- ithoughtsx format import
- Incremental state updates with state vectors
- Writing some tests
- Markdown export/import
- Richtext node notes
  - mermaid & gfm
- Dealing with overlapping trees somehow?
- Add action buttons for keyboard shortcuts
- Animations
- Version snapshots (to create different annotations)
- Bundling
- User auth for server?



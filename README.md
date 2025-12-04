# Development

## Run client in web 

``` bash
cd client
dx serve --platform web --features web
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
- Animations



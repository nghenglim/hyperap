# Hyperap
[![crates.io](http://meritbadge.herokuapp.com/hyperap)](https://crates.io/crates/hyperap)

Hyperap - Hyper wrapper. A very minimal wrapper for Hyper.rs to create a working webserver. 

## How To Use
code refer to `./src/main.rs`

to see how it works
```bash
git clone https://github.com/nghenglim/hyperap
cd hyperap
cargo run
```

## NOTE
- Currently using hyper 0.11 branch which is async in nature, however coding in async will not clean unless the async/await rust feature come out
- No point on creating synchronous version of rust webserver because NodeJS/Java/Go webserver is asynchronous version.

## TODO
- [ ] more functional on the add_route
- [ ] route is able to specify the GET/POST definition, do validation before go into controller
- [ ] all the routing is able to generate into a swagger file
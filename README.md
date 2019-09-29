# Heroku-rust-hello

## Installation
Deploying Rust applications to Heroku, with example code for Rustful

```bash
$ git clone https://github.com/ThePFMind/heroku-rust-hello.git
$ cd heroku-rust-hello
$ heroku login
$ heroku git:remote -a heroku-rust-hello
$ heroku create --buildpack emk/rust
$ heroku buildbpacks:set emk/rust
$ git push heroku master
```
## Combining with other buildpacks

```bash
$ heroku buildpacks:add --index 1 https://git.heroku.com/heroku-rust-hello.git
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
# gpt-j-ru
GPT-J client lib

## quickstart

```sh
cargo test -- --nocapture
```

```
running 1 test
ClassifyResponse { sequence: "In a shocking finding, scientists discovered a herd of unicorns living in a remote, previously unexplored valley, in the Andes Mountains. Even more surprising to the researchers was the fact that the unicorns spoke perfect English.", labels: ["positive", "neutral", "negative"], scores: [0.6839565, 0.25236782, 0.06367566] }
test tests::test_classify ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.87s
```

## Thanks

Thanks go to vicgalle for providing https://github.com/vicgalle/gpt-j-api and the api service
http://api.vicgalle.net:5000/docs.

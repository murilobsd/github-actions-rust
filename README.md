# Github Secret Rust

Simple example of how to publish secret to github actions.

# Quickstart

1. [Create personal token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
2. Get repo public key and key id, replace token owner and repo name.
```console
TOKEN=MY_PERSONAL_TOKEN OWNER= REPO= curl \
       -H "Accept: application/vnd.github.v3+json" \
       -H "Authorization: token $TOKEN" \
       https://api.github.com/repos/${OWNER}/${REPO}/actions/secrets/public-key
```
3. Get values and add to .env file
```console
cp .env_example .env
```
4. Run
```console
cargo run
```

## References

-
[https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)

## Generate keys and message

```
cargo run --bin simple-encrypt-decrypt
```
will generate message.pgp and secret.pgp

## Extract session key

```
cargo run --bin extract-session-key secret.pgp message.pgp
```

copy-paste the last printed line to the console `export KEY=xxxx`

## Extract the SEIP

```
cargo run --bin extract-seip-content message.pgp
```
copy-paste the openssl command and printout the decrypted.bin



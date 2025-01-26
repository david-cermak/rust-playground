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


----

# Step by step guide:

# Generate new PGP key:

```
gpg --full-generate-key
```
* choose RSA-2048
* no passphrase

## List keys and note the "long" key-id

```
 $ gpg --list-secret-key --keyid-format long

/home/david/.gnupg/pubring.kbx
------------------------------
sec   rsa2048/069ECFEC25327D7B 2025-01-26 [SC]
      E70262371AA68EF6D63D476B069ECFEC25327D7B
uid                 [ultimate] David Cermak (Personal) <xxxxx@xxxxxx.xxx>
ssb   rsa2048/69A16114A29E364C 2025-01-26 [E]
```
the long key-id is the number "069ECFEC25327D7B"
the ssb (subkey) id is "69A16114A29E364C" --> this will be used for converting to plain RSA keys

## Export to plain RSA key

```
gpg --export-secret-keys 69A16114A29E364C | openpgp2ssh 69A16114A29E364C > secret.pem

```

# Create the password file

```
echo "Super-secret-password" | gpg --compress-level 0 --encrypt --recipient 069ECFEC25327D7B  --output password.pgp
```

# Decrypt the password

## Get the encrypted session key

```
cargo run --bin extract-seip-content password.pgp
```
this parses openpgp packets and creates two files (encrypted binaries)
* `pkesk.bin` (asymmetrically encrypted session key)
* `encrypted.bin` (symmetrically encrypted message)

## Decrypt the session key

Lets decrypt the `pkesk.bin` directly with the RSA private key:

```
openssl rsautl -decrypt -in pkesk.bin -inkey secret.pem -out session_key.bin
```
check the session key:
```
~$ xxd session_key.bin
00000000: 09fa 2672 fd63 269a ab74 6e7d d8b5 f3ce  ..&r.c&..tn}....
00000010: 1e98 4d00 a25d ed0f efc3 0418 9d24 10af  ..M..].......$..
00000020: 850f db                                  ...
```
and compare with gpg extracted session key:
```
~$ gpg --show-session-key --decrypt password.pgp 
gpg: encrypted with 2048-bit RSA key, ID 69A16114A29E364C, created 2025-01-26
        .....
gpg: session key: '9:FA2672FD63269AAB746E7DD8B5F3CE1E984D00A25DED0FEFC304189D2410AF85'
```

## Use the session key to decrypt the message

export the session key
```
export KEY=FA2672FD63269AAB746E7DD8B5F3CE1E984D00A25DED0FEFC304189D2410AF85
```
and use the openssl command printed from `cargo run --bin extract-seip-content password.pgp`

```
~$ openssl enc -aes-256-cfb -d -K $KEY -iv D9218797E27FAD1931CFD8EB9E74EEE5 -in encrypted.bin
�
 ֋0L���=
v* �<�bg���Super-secret-password
�-�"���g
8�����E�~
```

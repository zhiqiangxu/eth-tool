## Instructions

```
# generate deposit signature
cargo run deposit sign --seckey 1acdedb801601e1b4d491d38775e7a3c5fb3318626906a4337c01085991b5700 --amount 10 --withdrawal-credentials abaaaaaaaaaaaaaaabaaaaaaaaabaaaaaaaaabaaaaaaaaabaaaaaaaaabaaaaaa
# generate deposit_data_root
cargo run deposit data-root --pubkey 0x94cbc1b96e0b3db4203b350f2316235187ed3a9229c2ca5eab6321f1e77737775597e075ad4d0b295ad7b0d42271f8a8 --amount 10 --withdrawal-credentials abaaaaaaaaaaaaaaabaaaaaaaaabaaaaaaaaabaaaaaaaaabaaaaaaaaabaaaaaa --signature 0x890347635620b878ec9aa6c62698d6275a6b07ebb5d670c6c9f0028514d522a1ac21104a7fd9785555cb2ac94e7f0a6f175f1cc4cac8231f56a7e55f9ceef07c02600973af66dc16555dce6198953f8dcbc4c5f31e794b0dc09729f54bc84917
```
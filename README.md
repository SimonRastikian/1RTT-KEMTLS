# Post-Quantum TLS without handshake signatures

This repository accompanies

* Felix Günther and Patrick Towa. **KEMTLS with Delayed Forward Identity Protection in (Almost) a Single Round Trip.** Draft paper
* Peter Schwabe, Douglas Stebila and Thom Wiggers. **More efficient KEMTLS with pre-distributed public keys.** Draft paper
* Peter Schwabe, Douglas Stebila and Thom Wiggers. **Post-quantum TLS without handshake signatures.** ACM CCS 2020.
* Peter Schwabe, Douglas Stebila and Thom Wiggers. **Post-quantum TLS without handshake signatures.** IACR Cryptology ePrint Archive, Report 2020/534. April 2021.

```
@inproceedings{CCS:SchSteWig20,
  author = {Schwabe, Peter and Stebila, Douglas and Wiggers, Thom},
  title = {Post-Quantum {TLS} Without Handshake Signatures},
  year = {2020},
  isbn = {9781450370899},
  publisher = {Association for Computing Machinery},
  address = {New York, {NY}, {USA}},
  url = {https://thomwiggers.nl/publication/kemtls/},
  doi = {10.1145/3372297.3423350},
  booktitle = {Proceedings of the 2020 {ACM} {SIGSAC} Conference on Computer and Communications Security},
  pages = {1461–1480},
  numpages = {20},
  keywords = {transport layer security, key-encapsulation mechanism, {NIST PQC}, post-quantum cryptography},
  location = {Virtual Event, {USA}},
  series = {{CCS '20}}
}

@online{EPRINT:SchSteWig20,
  author = {Peter Schwabe and Douglas Stebila and Thom Wiggers},
  title = {Post-quantum {TLS} without handshake signatures},
  year = 2021,
  month = apr,
  note = {full online version},
  url = {https://ia.cr/2020/534},
}
```

## Overview of this repository

### Main folders

* ``rustls-kemtls``: modified Rustls TLS stack to implement KEMTLS and post-quantum versions of "normal" TLS 1.3
* ``measuring``: The scripts to measure the above
* ``ring``: Modified version of Ring to allow for longer DER-encoded strings than typically expected from TLS instances.
* ``webpki``: Modified version of WebPKI to work with PQ and KEM public keys in certificates
* ``mk-cert``: Utility scripts to create post-quantum PKI for pqtls and KEMTLS.
* ``certificates``: Contains some pre-generated certificates only for testing.

### Supporting repositories

* [``oqs-rs``][]: Rust wrapper around ``liboqs``.
* ``mk-cert/xmss-rs``: Rust wrapper around the XMSS reference code, with our custom parameter set (``src/settings.rs``) and utilities for keygen and signing.
* ``csidh-rust``: Rust wrapper around the Meyer, Campos, Reith constant-time implementation of CSIDH.

[``oqs-rs``]: https://github.com/open-quantum-safe/liboqs-rust.git

## Working with this repository

* The Dockerfile serves as an example of how everything can be compiled and how test setups can be created.
   It is used by the ``./measuring/script/create-experimental-setup.sh`` script, which serves as an example of its use.
* The `mk-certs` folder contains a python script, `encoder.py`, that can be used to create the required PKI.
   RSA certificates and X25519 certificates are available in subfolders.
   The certificates assume that the server hostname is ``servername``, so put this in your `/etc/hosts`.
   Alternatively, override it using the environment variables in the file (which is also how you set which algorithms are used).
* Experimenting with ``rustls`` can be done directly; use the ``rustls-mio`` subfolders
   and run ``cargo run --example tlsserver -- --help`` or ``cargo run --example tlsclient -- --help``.
* The measurement setup is handled in the `measuring/` folder. See the `./run_experiment.sh` script.
* Processing of results is done by the `./scripts/process.py` folder. It expects a `data` folder as produced by `./scripts/experiment.py`.
* Downloading archived results can be done through the scripts in ``measuring/archived-results/``

## Running KEMTLS and its variants

### Requirements
* In order to run the implementation some requirements are needed.
	All the following commands are assumed to be ran on **Linux-like** systems.
* First install [``Rust``](https://www.rust-lang.org/tools/install) latest version.
* The compilation will not work if your C compilor *gcc* is older than *gcc 7.1*.
* Docker installation is not required to run the code on your local machine.
* Install libssl-dev (C crypto library), CMake (packaging installer), Clang and LLVM (cross compilor and linker);
	``sudo apt-get install -qq -y libssl-dev cmake clang-12 llvm-12``
* Please install *pipenv* (Python virtualenv manager) if you would like to generate certificates from `mk-cert` folder.

### Running the implementation

* Before you start compilation, you need to add in `/etc/hosts` the line ``127.0.0.1 servername``
	This allows the client to connect to the server called *servername* on your localhost
* Then go to `rustls/rustls-mio`. This is where the *clienttls* and *servertls* main functions exist.
* First compile the code and run help by typing ``cargo run --example tlsserver -- --help`` orw
	``cargo run --example tlsclient -- --help``. This will run the server/client and output all the available
	options that will allow you to either run tls 1.3, or KEMTLS, or KEMTLS-PDK, or KEMTLS-PDK-SS (a.k.a. 1RTT-KEMTLS)
* Now let's run the KEMTLS-PDK-SS (Key Encapsulation Mecanism TLS with pre-distributed keys and semi-static keys)
	
	Run the server by typing 
	
	``cargo run --example tlsserver -- --port 10001 --certs ../../certificates/1RTT-KEMTLS/kem.crt --key ../../certificates/1RTT-KEMTLS/kem.key --require-auth  --auth ../../certificates/1RTT-KEMTLS/client-ca.crt --1rtt-key ../../certificates/1RTT-KEMTLS/kem_ssrttkemtls.key  --1rtt-public ../../certificates/1RTT-KEMTLS/kem_ssrttkemtls.pub --1rtt-epoch ../../certificates/1RTT-KEMTLS/server.epoch  http``
	
	or depending on which sub-protocol one would like to run (check **KEMTLS with Delayed Forward Identity Protection in (Almost) a Single Round Trip**) you can also type
	
	``cargo run --example tlsserver -- --port 10001 --certs ../../certificates/1RTT-KEMTLS/kem.crt --key ../../certificates/1RTT-KEMTLS/kem.key --require-auth  --auth ../../certificates/1RTT-KEMTLS/client-ca.crt --1rtt-key ../../certificates/1RTT-KEMTLS/kem_ssrttkemtls.key  --1rtt-public ../../certificates/1RTT-KEMTLS/kem_ssrttkemtls.pub --1rtt-epoch ../../certificates/1RTT-KEMTLS/server.epoch  --1rtt-key-next ../../certificates/1RTT-KEMTSL/semistatic-epoch-2.key --1rtt-epoch-next ../../certificates/1RTT-KEMTSL/semistatic-epoch-2.epoch --1rtt-public-next ../../certificates/1RTT-KEMTSL/semistatic-epoch-2.pub http``
	
	In parallel, run the client with
	
	``cargo run --example tlsclient -- -p 10001 --http --cafile ../../certificates/1RTT-KEMTLS/kem.chain.crt --auth-key ../../certificates/1RTT-KEMTLS/client.key --auth-certs ../../certificates/1RTT-KEMTLS/client.crt --1rtt-pk ../../certificates/1RTT-KEMTLS/kem_ssrttkemtls.pub --1rtt-epoch ../../certificates/1RTT-KEMTLS/client.epoch --cached-certs ../../certificates/1RTT-KEMTLS/kem.crt servername``
	
	Depending on the client and server epoch numbers (if they are equal or different) one round trip KEMTLS-SS protocol is used
	or the two round trip KEMTLS-SS is used.
	
Depending on what subprotocol you chose, you should be seeing something like:
at the client side:
```
START: 1115 ns
CREATED PDK ENCAPSULATION: 187481 ns
CREATED PDK 1RTT-KEMTLS ENCAPSULATION: 216550 ns
CREATING KEYSHARES: 226597 ns
CREATED KEYSHARES: 260368 ns
DERIVED HS: 304487 ns
SENDING CHELO: 321083 ns
EMIT CERT: 600959 ns
RECEIVED SH: 2101841 ns
DECAPSULATING EPHEMERAL: 2170121 ns
DECAPSULATED EPHEMERAL: 2197520 ns
DECAPSULATING FROM CCERT: 2220955 ns
DECAPSULATED FROM CCERT: 2248128 ns
DERIVED MS: 2335480 ns
RECEIVED FINISHED: 2410501 ns
EMITTED FINISHED: 2455681 ns
WRITING TO SERVER: 2532896 ns
HANDSHAKE COMPLETED: 2540453 ns
RECEIVED SERVER REPLY: 2912962 ns
HTTP/1.0 200 OK
Connection: close
Hello world from rustls tlsserver
```

at the server side:
```
RECEIVED CLIENT HELLO: 82 ns
PDK DECAPSULATING FROM CERTIFICATE: 323267 ns
PDK DECAPSULATED FROM CERTIFICATE: 375516 ns
PDK 1RTT-KEMTLS DECAPSULATING FROM SEMISTATIC: 453210 ns
PDK 1RTT-KEMTLS DECAPSULATED FROM SEMISTATIC: 474026 ns
DERIVED HS: 610155 ns
IAMAHEREE
ENCAPSULATING TO EPHEMERAL: 800827 ns
ENCAPSULATED TO EPHEMERAL: 866040 ns
EMITTED SH: 958572 ns
DERIVED MS: 1036618 ns
WRITING TO CLIENT: 1118664 ns
RECEIVED FINISHED: 1741114 ns
READING TRAFFIC: 1877086 ns
HANDSHAKE COMPLETED: 1882143 ns
```



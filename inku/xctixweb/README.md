# TechEmpower Framework Benchmarks

[Round 18 results - TechEmpower Framework Benchmarks](https://www.techempower.com/benchmarks/#section=data-r18&hw=ph&test=query)

# test certs and keys

[Mini tutorial for configuring client-side SSL certificates.](https://gist.github.com/mtigas/952344)

```
$ openssl ecparam -genkey -name secp256r1 | openssl ec -out ca.key
$ openssl req -new -x509 -days 3650 -key ca.key -out src/ca.crt
$ openssl ecparam -genkey -name secp256r1 | openssl ec -out server-ec.key
#
# rustls pemfile format PKCS8
# Converting Openssl ECC Private key to PKCS#8 format
#
$ openssl pkcs8 -topk8 -nocrypt -in server-ec.key -out src/server.key
$ openssl req -new -key server-ec.key -out server.csr
$ openssl x509 -req -days 3650 -in server.csr -CA src/ca.crt -CAkey ca.key -set_serial 01 -out src/server.crt
```
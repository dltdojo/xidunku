# building a image and docker run it

```shell
$ cargo build
$ bash build.sh build-xterm
$ bash build.sh build-wasm-webfoo
$ docker build -t foo .
$ docker run -it --rm -v $PWD:/home/alice/host -p 8443:8443 foo
```

skaffold and k8s 

```shell
$ skaffold dev --port-forward
```


# TechEmpower Framework Benchmarks

[Round 18 results - TechEmpower Framework Benchmarks](https://www.techempower.com/benchmarks/#section=data-r18&hw=ph&test=query)

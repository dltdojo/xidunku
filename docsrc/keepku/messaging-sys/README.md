# How to test

## smtp

```
$ cd keepku/messaging-sys
$ skaffold dev -p smtpx1
$ skaffold dev -p smtpx2
```
## rabbitmq

- guest/guest http://localhost:15672 

```
$ skaffold dev -p rabbitmq-dev --port-forward
$ skaffold dev -p rabbitmq --port-forward
```

# TODO

- [x] smtpx1 from bar@locahost to foo@localhost
- [x] smtpx2 from bar@taichung.ku.default.svc.cluster.local foo@taipei.ku.default.svc.cluster.local
- [ ] rust grpc processor ? [hyperium/tonic: A native gRPC client & server implementation with async/await support.](https://github.com/hyperium/tonic)
- [ ] read [kaiwaehner/tensorflow-serving-java-grpc-kafka-streams: Kafka Streams + Java + gRPC + TensorFlow Serving => Stream Processing combined with RPC / Request-Response](https://github.com/kaiwaehner/tensorflow-serving-java-grpc-kafka-streams)
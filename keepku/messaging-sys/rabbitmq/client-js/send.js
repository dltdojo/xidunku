// npm i amqplib
const amqp = require('amqplib/callback_api');
//const MQ_URL="amqp://localhost";
var RABBIT_HOST = process.env.RABBITMQ_HOST
const MQ_URL = "amqp://foo:pass@" + RABBIT_HOST;
amqp.connect(MQ_URL, function (error0, connection) {
  if (error0) {
    throw error0;
  }
  console.log("create channel from ", MQ_URL);
  connection.createChannel(function (error1, channel) {
    if (error1) {
      throw error1;
    }
    var queue = 'hello';
    var msg = 'Hello world ' + new Date().toISOString();

    channel.assertQueue(queue, {
      durable: false
    });

    channel.sendToQueue(queue, Buffer.from(msg));
    console.log(" [x] Sent %s", msg);
  });

  setTimeout(function () {
    connection.close();
    process.exit(0)
  }, 500);

});

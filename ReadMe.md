# Descriotion

This software includes a selective forwarding unit server written in Rust and a react application as web client.
Here, we're going to explain media server's architecture which designed based on Mediasoup-library to anyone who wants to understand key points of written code.

In order to use host's resources and support more users, application creates workers in demanded area ( which area can be producer or consumer ) each worker runs on a single CPU core. 

The application splits in to two areas:

1- First area is Producer area which includes workers that are used for by room's producer routers ( which producers send media traffic to).

2- Second area is Consumer area which includes workers that are used by room's producer routers ( which consumers receive traffic from by connecting to it).

A person who wants to send video/audio (Also called producer) connects to a router constructed in producer area and the person connects to a router constructed in consumer area to receive audio/video (Also called consumer).
Routers of a room are responsible for forwarding media from a producer to other users or simply other routers via piping. Existed routers in producer area pipe media to routers inconsumer area so that users can consume from them.

At last a room maintains multiple producer/consumer routers and each router is managed by a worker.

## Server

In order to read more about server take a look at [here](server).

## Client

In order to read more about web-app take a look at [here](react).

# Architecture

Following diagram is application's signaling server architecture:

![alt text](https://github.com/mojtaba-motevali/Media-server/blob/master/assets/server_rust_architecture.jpg?raw=true)

Following diagram is application's mediasoup architecture:

![alt text](https://github.com/mojtaba-motevali/Media-server/blob/master/assets/Mediasoup-svg.svg?raw=true)

## License

This application is [MIT License](LICENSE)

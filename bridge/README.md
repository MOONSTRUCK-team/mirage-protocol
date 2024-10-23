# evm-icp-bridge

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v1.1.24. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.


### Docker image running

In order to run the docker image, you need to build the image first. You can do this by running the following command:

```bash
docker build -t evm-icp-bridge .
```

After the image is built, you can run the image by running the following command:

```bash
docker run --env-file ./.env -it evm-icp-bridge
```

The `--env-file` flag is used to pass the environment variables to the docker container.


### Connecting all the pieces

In order to properly test the bridge, both the EVM and ICP nodes need to be running, as well as the service itself. To do this, you can run the following commands:

```bash
docker compose up --build
```




### Setting up the ICP listener

ICP listener is a service that exposes the endpoint intended as the contact with the ICP network. Think about it as the webhook that will be called by the ICP side of the Bridge.

To setup the listener, it must utilize tls certificates.
For the testing purposes, the keys and certificates are located and can be used from the `tls` directory.

For the production, the certificates should be generated and stored in a secure way. Ideally, they would be injected into the bridge from the secure enviroment like secrets.

Test certificates are generated using `mkcert` tool

```bash
mkcert --install
mkcert example.com "*.example.com" example.test localhost 127.0.0.1 ::1
```

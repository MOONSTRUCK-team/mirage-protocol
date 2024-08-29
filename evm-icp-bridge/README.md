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

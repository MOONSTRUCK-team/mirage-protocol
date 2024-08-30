const server = Bun.serve({
    port: 3000,
    tls: {
        cert: Bun.file("example.com+5.pem"), // TODO: Populate the file path to the certificate
        key: Bun.file("example.com+5-key.pem"), // TODO: Populate the file path to the key
    },
    async fetch(req) {
        const path = new URL(req.url).pathname;

        // respond with text/html
        if (path === "/") return new Response("Welcome Home!");

        // receive JSON data to a POST request
        if (req.method === "POST" && path === "/api/post") {
            const data = await req.json();

            var msg = 'Hello World';
            if (data.name) {
                msg = 'Hello ' + data.name;
            }

            return new Response(msg);
        }

        // 404s
        return new Response("Page not found", { status: 404 });
    }
})

console.log(`Listening on ${server.url}`);
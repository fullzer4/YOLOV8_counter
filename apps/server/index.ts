import fastify from "fastify"

const server = fastify()

server.get("/", async (request, reply) => {
  return { hello: "world"}
})

server.listen({
  port: 8080,
})

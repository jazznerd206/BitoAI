// This code uses TypeScript syntax and types for the  http module. You'll need to have TypeScript installed to compile and run this code.

import * as http from 'http';

const hostname: string = 'localhost';
const port: number = 3000;

const server: http.Server = http.createServer((req: http.IncomingMessage, res: http.ServerResponse) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end('Hello, World!\n');
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
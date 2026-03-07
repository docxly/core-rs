import { createReadStream, existsSync } from "node:fs";
import { stat } from "node:fs/promises";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = path.dirname(fileURLToPath(new URL("../package.json", import.meta.url)));
const siteRoot = path.join(packageRoot, "site-dist");
const port = Number(process.env.PORT || 4173);
const host = process.env.HOST || "127.0.0.1";

const contentTypes = new Map([
  [".css", "text/css; charset=utf-8"],
  [".html", "text/html; charset=utf-8"],
  [".js", "text/javascript; charset=utf-8"],
  [".json", "application/json; charset=utf-8"],
  [".wasm", "application/wasm"],
]);

function resolveRequestPath(urlPath) {
  const normalized = urlPath === "/" || urlPath === "" ? "/index.html" : urlPath;
  const safePath = path.normalize(normalized).replace(/^([.][.][/\\])+/, "");
  return path.join(siteRoot, safePath.replace(/^\//, ""));
}

const server = http.createServer(async (req, res) => {
  const url = new URL(req.url || "/", `http://${host}:${port}`);
  const filePath = resolveRequestPath(url.pathname);

  if (!filePath.startsWith(siteRoot)) {
    res.writeHead(403);
    res.end("Forbidden");
    return;
  }

  if (!existsSync(filePath)) {
    res.writeHead(404);
    res.end("Not Found");
    return;
  }

  const fileStat = await stat(filePath);
  if (fileStat.isDirectory()) {
    res.writeHead(404);
    res.end("Not Found");
    return;
  }

  const contentType = contentTypes.get(path.extname(filePath)) || "application/octet-stream";
  res.writeHead(200, {
    "Content-Length": fileStat.size,
    "Content-Type": contentType,
    "Cache-Control": "no-cache",
  });
  createReadStream(filePath).pipe(res);
});

server.listen(port, host, () => {
  console.log(`docxly demo server listening at http://${host}:${port}`);
});

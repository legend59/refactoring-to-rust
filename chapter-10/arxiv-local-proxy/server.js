import express from "express";
import fetch from "node-fetch";

const app = express();
const PORT = 3000;

app.get("/api/query", async (req, res) => {
  try {
    const query = new URLSearchParams(req.query).toString();

    const upstreamUrl = `https://export.arxiv.org/api/query?${query}`;

    const upstreamResp = await fetch(upstreamUrl, {
      method: "GET",
      redirect: "follow",
      headers: {
        "User-Agent": "local-arxiv-proxy/1.0",
      },
    });

    const body = await upstreamResp.text();

    res.status(upstreamResp.status);
    res.set("Content-Type", "application/atom+xml; charset=utf-8");
    res.send(body);
  } catch (err) {
    console.error(err);
    res.status(502).send("Bad Gateway");
  }
});

app.get("/books/", async (req, res) => {
  try {
    const search = new URLSearchParams(req.search).toString();

    const upstreamUrl = `https://gutendex.com/books/?search=${search}`;

    const upstreamResp = await fetch(upstreamUrl, {
      method: "GET",
      redirect: "follow",
      headers: {
        "User-Agent": "local-gutendex-proxy/1.0",
      },
    });

    const body = await upstreamResp.text();

    res.status(upstreamResp.status);
    res.set("Content-Type", "application/json; charset=utf-8");
    res.send(body);
  } catch (err) {
    console.error(err);
    res.status(502).send("Bad Gateway");
  }
});

app.listen(PORT, () => {
  console.log(`arXiv local proxy running at http://127.0.0.1:${PORT}`);
});


require('dotenv').config();

const express = require("express");
const cors = require("cors");
const serveIndex = require("serve-index");
const path = require("path");
const app = express();

const localPath = path.resolve(process.env.LOCALCDN_DIRECTORY);
const localPort = process.env.LOCALCDN_PORT;

console.log(localPath, localPort);

app.options('*', cors());
app.use(cors());
app.use(express.static(localPath), serveIndex(localPath, {'icons': true}));
app.listen(localPort, () => console.log('Local CDN Started!'))

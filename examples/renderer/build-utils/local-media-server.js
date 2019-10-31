require('dotenv').config();

if(!process.env.DEVELOPER || process.env.DEVELOPER === "") {
    console.log("Local CDN: set [DEVELOPER] in .env");
    process.exit(0);
}

const paths = {
    david: (osPlatform) => {
        switch(osPlatform) {
            default: return `C:\\Users\\david\\Documents\\projects\\khronos\\glTF-Sample-Models\\2.0`
        }
    },
	david_laptop: (osPlatform) => {
        switch(osPlatform) {
            default: return `C:\\Users\\david\\Documents\\github\\khronos\\glTF-Sample-Models\\2.0`
        }
    },
}

const os = require('os');
const path = require('path');
const fs = require('fs');

const localPath = path.resolve(
    paths[process.env.DEVELOPER.toLowerCase()] (os.platform())
);

const express = require('express');
const cors = require('cors');
const serveIndex = require('serve-index');

const app = express();

app.options('*', cors());
app.use(cors());
app.use(express.static(localPath), serveIndex(localPath, {'icons': true}));


//If you change it here - also change:
//1. config/Config.ts
//2. build-utils/transform-css.js (if exists)
app.listen(4102, () => console.log('Local CDN Started!'))
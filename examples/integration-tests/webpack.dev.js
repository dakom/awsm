const common = require('./webpack.common.js');
const path = require('path');

const devSettings = {
    mode: "development",
    devtool: "inline-source-map",
}

//separate configurations
const browserConfig = Object.assign({}, common.browserConfig, devSettings, {
    devServer: {
        //contentBase: path.join(__dirname, "dist/"),
        contentBase: path.resolve(__dirname, './site'),
        compress: true,
        port: 3000,
        headers: { "Access-Control-Allow-Origin": "*" },
        historyApiFallback: {
            disableDotRule: true
        },
        watchContentBase: true,
    }
});

module.exports = [browserConfig];

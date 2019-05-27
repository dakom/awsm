const {baseConfig} = require('./webpack.common.js');
const path = require('path');

module.exports = Object.assign({}, baseConfig, {
    mode: "development",
    devtool: "inline-source-map",

    devServer: {
        //contentBase: path.join(__dirname, "dist/"),
        contentBase: path.resolve(__dirname, './public'),
        compress: true,
        port: 3000,
        headers: { "Access-Control-Allow-Origin": "*" },
        historyApiFallback: {
            disableDotRule: true
        },
        //watchContentBase: true,
    }
});

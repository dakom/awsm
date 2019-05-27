const path = require("path");
const webpack = require('webpack');
const HtmlWebpackPlugin = require("html-webpack-plugin");

const isProduction = process.env["NODE_ENV"] === "production";


module.exports = {
    baseConfig: {
        mode: isProduction ? "production" : "development",
        entry: "./js/index.js",
        output: {
            path: path.resolve(__dirname, "dist"),
            filename: "bundle.js"
        },
        module: {
            rules: [
                {
                    test: /\.css$/,
                    use: [{ loader: 'style-loader' }, { loader: 'css-loader' }],
                },
            ],
        },
        plugins: [
            new HtmlWebpackPlugin({
                template: 'index.html',
                title: "Awsm Examples"
            }),
            new webpack.DefinePlugin({
                'process.env': {
                    'NODE_ENV': JSON.stringify(process.env['NODE_ENV']),
                    'BUILD_VERSION': JSON.stringify(require("./package.json").version)
                }
            }),
        ]
    }
}

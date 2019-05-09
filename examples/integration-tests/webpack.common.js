const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');

const commonConfig = {
    module: {
        rules: [
            {
                //enforce: "pre",
                test: /\.tsx?$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "ts-loader", 
                        options: { transpileOnly: true }
                    },
                    "source-map-loader"
                ]
            },
            {
                test: /\.wasm$/,
                type: "webassembly/experimental",
            },
            {
                test: /\.scss$/,
                use: [ "style-loader" // creates style nodes from JS strings
                        , "css-loader" // translates CSS into CommonJS
                        , "sass-loader" // compiles Sass to CSS
                ]
            },
        ]
    },
    resolve: {
        extensions: [".tsx", ".ts", ".js", ".wasm"],
        alias: {
            "components": path.resolve(__dirname, "src/components/"),
            "config": path.resolve(__dirname, "src/config/")
        }
    },
}

const browserConfig = Object.assign({}, commonConfig, {
    entry: {
        io: path.resolve('./src/Main.ts'),
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "[name].bundle.js",
        sourceMapFilename: "[name].bundle.map",
        publicPath: '',
    },

    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, './site/index.html'),
            hash: true,
        }),

        new webpack.DefinePlugin({
            'process.env': {
                'NODE_ENV': JSON.stringify(process.env['NODE_ENV']),
                'BUILD_VERSION': JSON.stringify(require("./package.json").version)
            }
        }),
        new ForkTsCheckerWebpackPlugin()
    ],
});

//Could have more configs for workers
module.exports = {browserConfig}


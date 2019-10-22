/* eslint-disable @typescript-eslint/no-var-requires */
const path = require('path');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const TerserPlugin = require('terser-webpack-plugin');

module.exports = {
    mode: "production",
    optimization: {
		minimizer: [new TerserPlugin({
			parallel: true,
            //sourceMap: true
		})]
    },
    context: process.cwd(), // to automatically find tsconfig.json
    entry: "./typescript/entry/index.ts",
    output: {
        path: path.join(process.cwd(), 'dist'),
        filename: '[name].js',
    },
    plugins: [
        new ForkTsCheckerWebpackPlugin({
            async: false,
            useTypescriptIncrementalApi: true,
            memoryLimit: 4096
        }),
        new HtmlWebpackPlugin({
            hash: true,
            inject: true,
            template: 'typescript/entry/index.html',
            minify: {
                removeComments: true,
                collapseWhitespace: true,
                removeRedundantAttributes: true,
                useShortDoctype: true,
                removeEmptyAttributes: true,
                removeStyleLinkTypeAttributes: true,
                keepClosingSlash: true,
                minifyJS: true,
                minifyCSS: true,
                minifyURLs: true,
            },
        }),
    ],
    module: {
        rules: [
            {
                test: /.ts$/,
                use: [
                    { loader: 'ts-loader', options: { transpileOnly: true } }
                ],
            },
            {

                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ]
    },
    resolve: {
        extensions: [".ts", ".js", ".css"],
        alias: {
            "@events": path.resolve(__dirname, "typescript/events"),
            "@state": path.resolve(__dirname, "typescript/state"),
            "@ui": path.resolve(__dirname, "typescript/ui"),
            "@utils": path.resolve(__dirname, "typescript/utils"),
            "@config": path.resolve(__dirname, "typescript/config"),
        }
    }
};
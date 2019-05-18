const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const isProduction = process.env["NODE_ENV"] === "production";

let wasmPluginOptions = {
      crateDirectory: path.resolve(__dirname, "crate"),
}

if(isProduction) {
    wasmPluginOptions.forceMode = "release"
}

module.exports = {
    mode: isProduction ? "production" : "development",
  entry: "./js/index.js",
  output: {
    path: dist,
    filename: "bundle.js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),

    new WasmPackPlugin(wasmPluginOptions),
  ]
};

const {baseConfig} = require('./webpack.common.js');
const TerserPlugin = require('terser-webpack-plugin');
const path = require('path');

module.exports = Object.assign({}, baseConfig, {
    mode: "production",
    //devtool: "source-map",
    optimization: {
        minimizer: [new TerserPlugin({
            parallel: true,
            //sourceMap: true
        })]
    }
});

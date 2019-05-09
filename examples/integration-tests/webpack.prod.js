const common = require('./webpack.common.js');
const TerserPlugin = require('terser-webpack-plugin');

const prodSettings = {
    mode: "production",
    //devtool: "source-map",
    optimization: {
        minimizer: [new TerserPlugin({
            parallel: true,
            //sourceMap: true
        })]
    }
}

//separate configurations
const browserConfig = Object.assign({}, common.browserConfig, prodSettings);

module.exports = [browserConfig];

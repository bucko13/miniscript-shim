const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./src/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "miniscript-shim.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({patterns: ['index.html']})
  ],
  experiments: {
    asyncWebAssembly: true,
  }
};
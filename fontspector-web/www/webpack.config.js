const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const crypto = require("crypto");
const { experiments } = require("webpack");
const crypto_orig_createHash = crypto.createHash;
crypto.createHash = algorithm => crypto_orig_createHash(algorithm == "md4" ? "sha256" : algorithm);

module.exports = {
  entry: "./bootstrap.js",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
    fallback: {
      "path": false,
      "fs": false,
    }
  },
  output: {
    path: path.resolve(__dirname, "..", "..", "docs"),
    filename: "bootstrap.js"
  },
  experiments: {
    asyncWebAssembly: true
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
        'index.html',
        '*.css',
        '*.png',
        'cmark-gfm.js',
        'node_modules/harfbuzzjs/hb.wasm',
       ]
    )
  ]
};

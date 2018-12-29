const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
  entry: './bootstrap.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bootstrap.js',
    globalObject: 'self',
  },
  mode: 'development',
  // module: {
  //   rules: [
  //     {
  //       test: /.wasm$/,
  //       // This is needed to make webpack NOT process wasm files.
  //       // See https://github.com/webpack/webpack/issues/6725
  //       type: 'javascript/auto',
  //       loader: 'file-loader',
  //       options: {
  //         name: '[name].[hash:5].[ext]',
  //       },
  //     },
  //   ],
  // },
  optimization: {
    // We no not want to minimize our code.
    minimize: false,
  },
  plugins: [new CopyWebpackPlugin(['index.html'])],
};

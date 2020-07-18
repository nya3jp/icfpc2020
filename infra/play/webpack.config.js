const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  mode: 'development',
  entry: {
    cli: './src/cli.ts',
    web: './src/web.ts',
  },
  resolve: {
    extensions: ['.ts', '.js'],
  },
  module: {
    rules: [
      { test: /\.tsx?$/, loader: 'ts-loader' },
    ],
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, 'dist'),
  },
  plugins: [
    new CopyPlugin({
      patterns: [{
        from: 'static/index.html',
        to: 'index.html',
      }],
    }),
  ],
};

const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

const distDir = path.resolve(__dirname, 'dist');

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
    path: distDir,
  },
  plugins: [
    new CopyPlugin({
      patterns: [{
        from: 'static/index.html',
        to: 'index.html',
      }],
    }),
  ],
  devServer: {
    contentBase: distDir,
    inline: false,
  },
};

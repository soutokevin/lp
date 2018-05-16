const path = require('path')
const HtmlPlugin = require('html-webpack-plugin')

module.exports = {
  entry: './web/main.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js'
  },
  plugins: [new HtmlPlugin({ template: 'web/index.html' })],
  mode: 'development'
}

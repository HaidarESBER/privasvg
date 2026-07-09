const path = require('path');

module.exports = {
  entry: {
    main: "./bootstrap.js",
    bulk: "./bulk.js"
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].bundle.js",
    // 'auto' resolves chunks/assets relative to the document, so the build
    // works both at the dev-server root and under a GitHub Pages subpath.
    publicPath: 'auto'
  },
  mode: "development",
  devServer: {
    port: 8080,
    static: {
      directory: path.join(__dirname),
      watch: true
    },
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin'
    }
  },
  resolve: {
    extensions: ['.js', '.wasm', '.mjs'],
    fallback: {
      "fs": false,
      "path": false,
      "crypto": false
    }
  },
  experiments: {
    asyncWebAssembly: true
  }
};


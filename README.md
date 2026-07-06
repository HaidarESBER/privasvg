<div align="center">

# PrivaSVG

### Turn any image into clean, scalable SVG — privately, in your browser.

**AI background removal · manual retouch · upscaling · true-color & B/W vectorization · bulk processing**

[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-0b5e3b.svg)](#license)
![Rust](https://img.shields.io/badge/Rust-WASM-0b5e3b)
![WebAssembly](https://img.shields.io/badge/WebAssembly-0b5e3b)
![Client-side](https://img.shields.io/badge/100%25-client--side-0b5e3b)

</div>

---

## What is PrivaSVG?

PrivaSVG is a **fully client-side raster → vector pipeline**. Drop in a `PNG`, `JPG`, or
`WEBP`, optionally strip the background with an in-browser AI model, retouch and upscale it,
then trace it to a crisp `SVG` — without uploading a single byte to a server.

Privacy is the point: the vectorization engine is Rust compiled to **WebAssembly**, the
background-removal model runs on **ONNX Runtime Web**, and every pixel is processed on your
own device. No accounts, no uploads, no tracking.

## The pipeline

```
Raster image  ->  AI background   ->  Manual retouch  ->  Upscale   ->  Vectorize    ->  SVG
PNG / JPG /       removal              remove / restore    2x / 4x      Rust -> WASM      output
WEBP
```

Every stage is optional except the last — jump straight to vectorization, or run the whole chain.

## Features

| Feature | Details |
|---|---|
| **AI background removal** | `@imgly/background-removal` (`isnet_fp16` on ONNX Runtime Web), with a residue-cleanup pass that removes low-alpha "ghost" pixels |
| **Manual retouch** | Brush-based remove/restore to fix edges the model missed, with adjustable brush size |
| **Upscaling** | 2x / 4x high-quality canvas resampling to recover detail before tracing |
| **True-color vectorization** | Incremental clustering into many colored `<path>` layers; stacked or cutout hierarchy |
| **B/W vectorization** | Threshold to a binary image and trace filled shapes |
| **Transparency-aware** | Uses a discarded key color so transparent regions survive into the SVG |
| **Flexible export** | Download as SVG, PNG, or a background-free raster |
| **Bulk mode** | Batch up to 50 images through the full pipeline and download a single ZIP |

## Running locally

**Prerequisites:** [Rust](https://www.rust-lang.org/tools/install) (with the
`wasm32-unknown-unknown` target), [`wasm-pack`](https://rustwasm.github.io/wasm-pack/), and
[Node.js](https://nodejs.org/) + npm. See [INSTALL.md](INSTALL.md) for detailed setup and
troubleshooting.

```bash
# From the repository root
./build.sh              # builds the WASM package into app/www/pkg and installs npm deps
cd app/www
npm start
```

Then open <http://localhost:8080>. The bulk page is at `bulk.html`.

<details>
<summary>Manual build (without <code>build.sh</code>)</summary>

```bash
cd app
wasm-pack build --target web --out-dir www/pkg
cd www
npm install
npm start          # or: npm run build  (production bundle)
```
</details>

## Tuning the tracer

Both clustering modes expose the same knobs through the Vectorization Settings panel:

| Parameter | What it does |
|---|---|
| Clustering | Color (true color) or B/W (binary) |
| Hierarchical | Stacked (layers overlap) or Cutout (disjoint shapes) — color mode |
| Curve fitting | Pixel · Polygon · Spline |
| Filter Speckle | Discard patches smaller than N px (denoise) |
| Color Precision | Significant bits per RGB channel |
| Gradient Step | Color distance between layers |
| Corner Threshold | Minimum angle to count as a corner |
| Segment Length | Subdivide-and-smooth until segments are shorter than N |
| Splice Threshold | Minimum angle to splice a spline |

## Design

PrivaSVG uses an editorial, privacy-forward visual language shared with
[PrivaSearch](https://github.com/HaidarESBER/privasearch): a cream-and-deep-green palette,
serif display type, monospace labels, generous whitespace, and a matching light/dark theme.

## How it works

- **Vectorization core** — a Rust engine (VTracer lineage) exposing `ColorImageConverter` and
  `BinaryImageConverter`, compiled to WebAssembly with `wasm-bindgen`. It clusters pixels,
  fits curves, and writes `<path>` elements directly into the target `<svg>`.
- **Background removal** — `@imgly/background-removal` runs the `isnet_fp16` segmentation model
  via ONNX Runtime Web; the model is fetched once, cached, then works offline.
- **Everything is local** — images never leave the browser; there is no backend.

## Tech stack

Rust · WebAssembly (`wasm-bindgen`, `web-sys`) · visioncortex / VTracer ·
ONNX Runtime Web (`@imgly/background-removal`) · JSZip · webpack

## Credits

- [VTracer](https://github.com/visioncortex/vtracer) & [visioncortex](https://github.com/visioncortex/visioncortex) — the Rust tracing + clustering foundation
- [`@imgly/background-removal`](https://github.com/imgly/background-removal-js) — in-browser AI background removal
- [JSZip](https://stuk.github.io/jszip/) — ZIP bundling for bulk exports

## License

Dual-licensed under **MIT OR Apache-2.0** — see [`LICENSE-MIT`](LICENSE-MIT) and
[`LICENSE-APACHE`](LICENSE-APACHE).

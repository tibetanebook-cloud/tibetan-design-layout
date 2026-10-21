---
title: Open a document
description: Open Tibetan Design Layout files, InDesign IDML packages, or import InDesign .indd and .indt files for editing.
order: 20
verified: 0.9.1
---

Choose **File**, then **Open…** (Ctrl+O), or click **Open** on the start screen. Tibetan Design Layout opens its own `.designcraft` documents and InDesign Markup (`.idml`) files. It also converts InDesign documents (`.indd`) and templates (`.indt`) to IDML during import so you can edit the converted layout.

Shortcuts on this page use Ctrl. On macOS, use Cmd instead of Ctrl.

## Steps

1. Open the **File** menu and choose **Open…**. On the start screen you can click **Open** instead.
2. Pick a `.designcraft`, `.idml`, `.indd`, or `.indt` file. The app may show an import warning if a package part or font cannot be read.
3. The file opens as a new, editable document. Review the pages and any import warnings.

You can also drag and drop these document types into the app window. To place an image, PDF, or text file inside an existing document, use [Place](place-images-and-text.md) instead.

## InDesign packages, fonts, and linked images

IDML is InDesign's interchange format. For `.indd` and `.indt`, Tibetan Design Layout converts the source to IDML first, then imports that layout. Keep the document's `Links` folder and any linked assets next to the source file when possible. If a `Document Fonts` folder sits beside the file, the app can load those fonts for the document.

Text that uses a font the app cannot find is highlighted in pink. Install the font or put its permitted font files in a `Document Fonts` folder beside the document, then reopen it.

Import conversion may not preserve every InDesign feature or reproduce the original layout exactly. Real-world `.indd` files have not yet been tested as a representative corpus, so inspect the converted pages before production use. If an IDML package is missing parts, the app shows an **Import Warnings** alert. If it has no spreads, there is no page content to open.

## Save or return to InDesign

**File**, then **Save** (Ctrl+S) writes the document in Tibetan Design Layout's `.designcraft` format. The app does not write native `.indd` or `.indt` files. To continue in InDesign, choose **File**, then **Export IDML…**, and open that `.idml` in Adobe InDesign.

---
title: Can Tibetan Design Layout open Adobe InDesign files?
description: Open .indd, .indt, and .idml files for editing, then save as .designcraft or export as IDML.
order: 30
verified: 0.9.1
---

Yes. Tibetan Design Layout can open InDesign Markup (`.idml`) files directly. It can also convert InDesign documents (`.indd`) and templates (`.indt`) to IDML during import, then open the converted layout for editing.

Choose **File**, then **Open…**, and select the file. You can also drop an `.indd` or `.indt` file into the app window. See [Open a document](../guides/open-a-file.md) for details.

The imported document is editable in Tibetan Design Layout. **Save** or **Save As** stores it in the app's `.designcraft` format. To continue editing in Adobe InDesign, use **File**, then **Export IDML…**, and open the resulting `.idml` file in InDesign. Tibetan Design Layout does not save directly to Adobe's native `.indd` or `.indt` formats.

Import conversion is not guaranteed to preserve every feature or match InDesign pixel for pixel. The project has not yet been validated against a representative set of real-world `.indd` files. Review the converted pages before relying on them, especially for complex layouts. Missing fonts and linked assets can also affect appearance; keep a `Document Fonts` folder and the document's `Links` folder with the source when available.

Tibetan Design Layout can also export PDF, PNG, and EPUB. See [Export a PDF](../guides/export-a-pdf.md).

# Tibetan template library

Created by ནང་ཆེན་བཀྲ་ཤིས།.

Choose File → Template Library. Each template opens a new native document with editable text; borders and image placeholders are editable frames. Replace the placeholder text, add pages and thread frames as needed. The sample folio is editable text, not automatic numbering. Pecha presets include title and front/back leaves; book presets are single pages; duplex imposition and automatic Tibetan folio numbering are not implemented by this library.

| Template | Trim size (mm) | Text size (pt) |
|---|---:|---:|
| Pecha — Long 420 × 100 mm | 420 × 100 | 28 |
| Pecha — Classic 300 × 100 mm | 300 × 100 | 27 |
| Pecha — Compact 210 × 74 mm | 210 × 74 | 22 |
| Pecha — Large 500 × 120 mm | 500 × 120 | 30 |
| Pecha — Title Leaf 300 × 100 mm | 300 × 100 | 30 |
| Pecha — Tibetan / English 300 × 140 mm | 300 × 140 | 24 |
| Pecha — Commentary 420 × 140 mm | 420 × 140 | 27 |
| Pecha — Unframed 300 × 100 mm | 300 × 100 | 27 |
| Book — Tibetan Reading A5 | 148 × 210 | 18 |
| Book — Paperback 6 × 9 in | 152.4 × 228.6 | 18 |
| Book — Bilingual A5 | 148 × 210 | 17 |
| Book — Study Workbook A4 | 210 × 297 | 20 |
| Book — Poetry 5.5 × 8.5 in | 139.7 × 215.9 | 18 |
| Book — Illustrated 8 × 10 in | 203.2 × 254 | 20 |
| Book — Dictionary A4 Two Columns | 210 × 297 | 16 |
| Book — Title Page 6 × 9 in | 152.4 × 228.6 | 28 |

Pecha sizes here are practical starting choices, not a claim of a universal traditional size. The long horizontal leaves use 12 mm margins; books use 18 mm. Bilingual layouts have separate Tibetan and English frames. The illustrated book includes an image placeholder. Pecha title leaves and book title pages are centered. Check overset text and stacked Tibetan glyph spacing before printing.

## References and design choices

- W3C, Requirements for Tibetan Text Layout and Typography, §8: https://www.w3.org/TR/2024/DNOTE-tlreq-20240419/ . Pecha text areas may be framed or unframed; traditional body type is often around 27–30 pt. Original layouts here include framed, unframed and compact options.
- Amazon KDP, Set Trim Size, Bleed, and Margins: https://kdp.amazon.com/en_US/help/topic/GVBQ3CMEQW3W2VL6 . 6 × 9 inches is a common paperback trim. Binding margins depend on final page count, paper and printer. The starting margins are not a guarantee of publisher acceptance; adjust the inside margin for the final binding and page count. Covers and spines require printer-specific measurements and are not included.
- Reviewed guimaluf/tibetan-pecha and CTAN Pecha for available formats. No code, liturgical texts, artwork, fonts or templates from those projects are bundled. This library is original code under the inherited project code licenses.

This is a curated collection; there is no exhaustive library of all Pecha or book templates. Tibetan proofreading and print proofing are still required.


## Revised Pecha layouts

Pecha presets now create a title leaf plus front/back body leaves (the title-only preset remains one page). Double editable black borders enclose separate rotated short-title, body, and Tibetan folio panels, inside a fine outside rule. Traditional presets use 27–30 pt Tibetan text with 1.65× leading; the compact preset uses 22 pt. Bilingual and commentary leaves have a dividing rule and separate editable text frames. Change the short title, author, folio labels, and body text before printing.

These are original layouts based on the user-provided PechaMaker screenshot and the [PechaMaker layout documentation](https://www.pechamaker.com/FAQ.html). No PechaMaker code, fonts, or copyrighted sample text is bundled. Leaves use manual editable folio labels; adding more leaves does not automatically update numbering or thread text between them. The templates are not printer imposition sheets.

The top menu now includes **Language** immediately before Help; English and བོད་ཡིག · Tibetan are available there as well as through Edit → Interface Language.

## Layout 0.8.2 application update

The supplied endless-knot icon is used by the Windows executable and installer, Mac Dock and app bundle, welcome screen and About window. The Tibetan product name is **བོད་ཀྱི་ཤོག་ངོས་བཟོ་བཀོད།**.

Choose **English / བོད་ཡིག** beside Help to switch languages. The existing Pecha and book library remains under File → Template Library.

The desktop app checks the official GitHub release metadata after startup and then daily. Help → Check for Updates checks manually. About includes an automatic-check opt-out. A newer compatible stable release offers Open download page, Later and Skip this version. It never downloads or runs an installer automatically. Existing installations must install this release once to receive these notifications.

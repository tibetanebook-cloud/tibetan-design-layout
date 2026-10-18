# Tibetan Design Layout

**བོད་ཀྱི་ཤོག་ངོས་བཟོ་བཀོད།**

![StudyTibetan logo](assets/app-icon/designcraft-1024.png)

Free native page layout and publishing software customized from DesignCraft, with Tibetan and English interface options and StudyTibetan branding.

Choose **Edit → Interface Language → བོད་ཡིག · Tibetan**. Choose English from the same menu to switch back. Tibetan menus and common controls are translated; uncovered strings remain English. Translation review and layout testing are ongoing.

Create page layouts with text and image frames, pages, styles, tables and PDF export. The supplied Noto Serif Tibetan font is embedded in packaged builds for Tibetan text and interface coverage.

## Downloads and builds

Run **Tibetan Design Layout builds** in GitHub Actions. Windows x64 produces an MSI and portable ZIP. Apple Silicon and Intel Mac jobs produce separate DMGs. Packages require successful build and verification before use. Without signing credentials, Windows packages are unsigned and Mac packages are ad-hoc signed, without Apple notarization.

Locally, prepare the font with `scripts/prepare-tibetan-font.ps1` (PowerShell) or source `scripts/prepare-tibetan-font.sh` (Bash), then build with Cargo. Native packaging scripts are in `packaging/windows` and `packaging/macos`.

More apps: [Nangchen](https://nangchen.com) · [StudyTibetan](https://studytibetan.com) · [TibetanEbook](https://tibetanebook.com).

## Credits and licenses

Based on [DesignCraft](https://github.com/storytold/designcraft), copyright its original authors. Original code remains MIT OR Apache-2.0; see LICENSE-MIT, LICENSE-APACHE and NOTICE. Required upstream contributor credits are retained. Supplied StudyTibetan branding has no separate public asset license asserted. Noto Serif Tibetan is under SIL OFL 1.1; see assets/app-icon/OFL-Tibetan.txt. See ASSETS.md for asset provenance.

Created by **ནང་ཆེན་བཀྲ་ཤིས།**.

## Preloaded templates

Choose **File → Template Library** for 8 Pecha and 8 book layouts. These are original editable starting layouts, not reproductions of third-party books. See [template guide](docs/tibetan-templates.md).

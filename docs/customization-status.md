# Customization status — 2026-10-15

Created by ནང་ཆེན་བཀྲ་ཤིས།. Source: storytold/designcraft at a930276fd7462f3d9abd2fbb38544c94c0c911c8.

- Product: Tibetan Design Layout, native Rust UI; Windows x64, Mac Apple Silicon and Intel build workflow.
- Tibetan language is selectable alongside English. Dictionary-guided catalog: 1,135 entries; untranslated strings fall back to English. Linguistic review is pending.
- User-supplied StudyTibetan logo in app/About, native icons and installers; distinct MSI upgrade identity, Mac bundle ID and Windows/Mac profile paths.
- 16 original editable templates: 8 Pecha presets and 8 book pages. Revised Pecha sets have a title leaf plus front/back body leaves, double borders, rotated short titles, and folio panels. Starting layouts, not automated full-book pagination or print imposition. See tibetan-templates.md.
- Noto Serif Tibetan is an external verified build input, registered for UI and native document text. Font binaries are ignored, not tracked.
- Original contributor credits and MIT/Apache notices retained; font license shipped in packages.
- Local native builds and GitHub installer verification must pass before claiming ready-to-install packages. Mac notarization and Windows signing need the owner's signing credentials.

- Pecha revision: Language top menu placed immediately before Help; Edit language submenu retained. Local validation and replacement installers are pending; the existing 0.8.0 downloads do not include this revision.

- 0.8.1 build repair: compact title folio panel text area widened from 5.5 mm to 10 mm while preserving border clearance. Overflow regression checks every Pecha preset before reporting failures. Remote validation pending.

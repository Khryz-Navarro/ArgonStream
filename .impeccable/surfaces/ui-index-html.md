---
version: 1
slug: "ui-index-html"
primary_target: "ui/index.html"
related_targets: ["ui/styles.css","ui/renderer.js"]
---

# App window (New download and Downloads tabs)

Scope: the whole app UI in `ui/` (Rust/Tauri build first; the Electron renderer follows). Mode: Operate.
Audience and job: the public on Windows 11 pasting a YouTube link, choosing a quality, and watching downloads
finish. Constraints: keep the two-tab layout, every control, all copy and behavior; dark by default with
Light and Desktop themes; usable at 100–175% page size; native-looking window buttons.

## Direction contract

THESIS: The app is a darkroom contact sheet: every download is a strip of film coming out of the developer. It refuses the graphite-panels-plus-one-blue default the old look was.

OWN-WORLD: Warm darkroom black; the title and status bars and the Quality column sit on their own darkroom tints instead of borders. One negative-mask orange marks selection, progress, focus and the active count; fixer green means done, safelight red means failed. Segoe UI Variable for words, Bahnschrift semi-condensed for edge codes, chips and figures. 3–4px corners, one-pixel rules, three ink values per theme. Light theme is the light table.

STORY: Paste a link, see it framed as a film frame with its edge code, pick a quality, download; on Downloads, watch strips develop and read done or failed at a glance.

FIRST VIEWPORT: Title bar field with an orange logo tile and tabs underlined in orange. Left: link box, preview as a film frame with a perforated rebate and an edge-code line (ID · best resolution · length; the channel sits under the title), folder row, print-white Download. Right, on the light-table tint: orange QUALITY label in Bahnschrift, six rows. Status bar field.

FORM: Film contact sheet and edge codes, candidate 4 of 7. seed:65c3bcbedc111697. Signature interaction: the edge strip, a download's progress drawn as a row of sprocket perforations that develop from dark to orange as bytes arrive, with the edge code and a large Bahnschrift percentage beside it; reduced motion shows the same state without easing.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance

## Open decisions

- The Electron renderer gets the same change once the other session's work there is merged.
- Short windows (150% and 175% page size) turn the film frame sideways so Download stays in the first viewport.

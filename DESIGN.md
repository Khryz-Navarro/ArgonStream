---
name: YT Downloader
description: A Windows 11 app that turns a YouTube link into a video file, drawn as a darkroom contact sheet.
colors:
  bg: "#13110F"
  field: "#0B0A09"
  table: "#1B1815"
  panel: "#1E1A17"
  surface: "#24201C"
  surface-2: "#2E2924"
  hover: "#3A342D"
  border: "#2E2924"
  border-2: "#4A4239"
  text: "#F2EDE4"
  muted: "#ADA497"
  faint: "#A08F7B"
  accent: "#F08A3C"
  accent-weak: "rgba(240, 138, 60, .12)"
  on-accent: "#13110F"
  ok: "#8FC79A"
  warn: "#E6C35C"
  err: "#F07560"
  film: "#070605"
  hole: "#2E2924"
  primary-hover: "#FFFFFF"
  bg-light: "#F2F1EE"
  field-light: "#E5E3DE"
  table-light: "#E9E7E2"
  panel-light: "#FBFAF8"
  surface-light: "#FFFFFF"
  surface-2-light: "#E3E0DA"
  hover-light: "#D8D4CC"
  border-light: "#DCD8D1"
  border-2-light: "#BDB6AB"
  text-light: "#1A1714"
  muted-light: "#57504A"
  faint-light: "#6D6255"
  accent-light: "#A0480B"
  accent-weak-light: "rgba(160, 72, 11, .09)"
  on-accent-light: "#FFFFFF"
  ok-light: "#2C7A43"
  warn-light: "#8C6300"
  err-light: "#BF301C"
  film-light: "#1A1714"
  hole-light: "#F2F1EE"
  primary-hover-light: "#000000"
typography:
  figure:
    fontFamily: "Bahnschrift SemiCondensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "20px"
    fontWeight: 600
    lineHeight: "22px"
    letterSpacing: "0.02em"
    fontFeature: "tnum"
  headline:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "15px"
    fontWeight: 600
    lineHeight: "20px"
  action:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "15px"
    fontWeight: 700
    lineHeight: "18px"
  title:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "13.5px"
    fontWeight: 600
    lineHeight: "18px"
  body:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: "18px"
  control:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "12.5px"
    fontWeight: 500
    lineHeight: "18px"
  body-small:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: "16px"
  label:
    fontFamily: "Bahnschrift SemiCondensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "12px"
    fontWeight: 600
    letterSpacing: "0.14em"
  chip:
    fontFamily: "Bahnschrift SemiCondensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "12px"
    fontWeight: 600
    lineHeight: "14px"
    letterSpacing: "0.08em"
  edge-code:
    fontFamily: "Bahnschrift SemiCondensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: "14px"
    letterSpacing: "0.06em"
    fontFeature: "tnum"
  count:
    fontFamily: "Bahnschrift SemiCondensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "11px"
    fontWeight: 600
    lineHeight: "16px"
    letterSpacing: "0.04em"
    fontFeature: "tnum"
rounded:
  frame: "1px"
  film: "2px"
  sm: "3px"
  md: "4px"
  round: "50%"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  inset: "14px"
  lg: "16px"
  dialog: "22px"
  xl: "24px"
  gutter: "32px"
  column-gap: "64px"
components:
  button-primary:
    backgroundColor: "{colors.text}"
    textColor: "{colors.bg}"
    typography: "{typography.action}"
    rounded: "{rounded.md}"
    height: "48px"
  button-primary-hover:
    backgroundColor: "{colors.primary-hover}"
  button-primary-disabled:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.faint}"
  button-secondary:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.text}"
    typography: "{typography.control}"
    rounded: "{rounded.sm}"
    padding: "0 12px"
    height: "32px"
  button-secondary-hover:
    backgroundColor: "{colors.hover}"
  button-icon:
    textColor: "{colors.muted}"
    rounded: "{rounded.sm}"
    size: "32px"
  button-icon-hover:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.text}"
  input-link:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "0 6px 0 14px"
    height: "44px"
  save-row:
    backgroundColor: "{colors.panel}"
    textColor: "{colors.muted}"
    rounded: "{rounded.md}"
    padding: "0 6px 0 12px"
    height: "44px"
  title-bar:
    backgroundColor: "{colors.field}"
    height: "40px"
  tab:
    textColor: "{colors.muted}"
    typography: "{typography.control}"
  tab-selected:
    textColor: "{colors.text}"
  count:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.muted}"
    typography: "{typography.count}"
    rounded: "{rounded.sm}"
    padding: "0 5px"
  count-active:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.on-accent}"
  quality-option:
    textColor: "{colors.text}"
    padding: "6px 14px"
    height: "48px"
  quality-option-selected:
    backgroundColor: "{colors.accent-weak}"
    rounded: "{rounded.md}"
  chip:
    textColor: "{colors.faint}"
    typography: "{typography.chip}"
  segmented-option:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.muted}"
    rounded: "{rounded.md}"
    height: "34px"
  segmented-option-selected:
    backgroundColor: "{colors.accent-weak}"
    textColor: "{colors.text}"
  preview-frame:
    backgroundColor: "{colors.film}"
    rounded: "{rounded.film}"
  download-item:
    backgroundColor: "{colors.panel}"
    rounded: "{rounded.md}"
    padding: "12px 12px 12px 14px"
  edge-strip:
    backgroundColor: "{colors.surface-2}"
    height: "8px"
  edge-strip-fill:
    backgroundColor: "{colors.accent}"
  edge-strip-fill-done:
    backgroundColor: "{colors.ok}"
  edge-strip-fill-error:
    backgroundColor: "{colors.err}"
  dialog:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    rounded: "{rounded.md}"
    padding: "22px"
    width: "460px"
  popover:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    rounded: "{rounded.md}"
    padding: "16px"
    width: "264px"
  status-bar:
    backgroundColor: "{colors.field}"
    textColor: "{colors.faint}"
    typography: "{typography.body-small}"
    padding: "0 16px"
    height: "32px"
---

# Design System: YT Downloader

## Overview

**Creative North Star: "Contact Sheet"**

YT Downloader is a darkroom contact sheet. The window is warm darkroom black; the video you paste is printed as a single frame on a strip of film, sprocket holes along its top rebate and its edge code (video ID · best resolution · length) printed underneath. On Downloads every download is its own strip coming out of the developer: a row of sprocket holes that develops orange as bytes arrive, ends fixer green when the file lands, and stops in safelight red where a download failed. The light theme is the light table: the same sheet laid on a lit, warm off-white surface, with the film itself staying dark.

The feel is quiet, exact and tactile. Regions are separated by tinted fields, not lines: the title bar and status bar sit on a darker field, and the Quality column sits on its own light-table tint beside the darkroom ground. One negative-mask orange carries every meaning that has to stand out: what is chosen, what is moving, what has focus, how many downloads are active. Words are set in Windows' own Segoe UI Variable; everything printed on film or counted (edge codes, chips, labels, counts, versions and each row's big number) is set in Bahnschrift semi-condensed, the edge-print voice. Corners are tight (1 to 4px), rules are one pixel, and each theme has three ink values.

Two looks are rejected: the ad-site downloader (big green DOWNLOAD buttons, badges, flashing banners) and the neon gamer (glows, gradients, RGB, glassy blur). This look also replaces the earlier graphite-panels-plus-one-blue look, which is retired.

**Key Characteristics:**
- Dark by default, warm darkroom black; a full light-table theme under the same token names.
- Regions divided by tinted fields (field, table) instead of borders.
- One negative-mask orange for selection, progress, focus and the active count.
- The single primary action is print white on darkroom black (ink on paper in light), never orange.
- Two voices: Segoe UI Variable for words, Bahnschrift semi-condensed for edge codes and figures.
- The edge strip: progress drawn as whole sprocket holes on a 14px pitch.
- Tight corners (1–4px), one-pixel rules, flat at rest.

## Colors

A warm brown-black palette with one orange, three developer-chemistry status colors, and a film black that stays dark in both themes; every token is defined twice, dark on `:root` and light under `prefers-color-scheme: light`, as `name` / `name-light` pairs.

### Primary
- **Negative-Mask Orange** (`accent` / `accent-light`, a burnt sienna on the light table): the selected quality row's border, the radio dot, the developing sprocket holes, the selected tab's underline, focus outlines, the link box caret, the active Downloads count, the logo tile and the QUALITY label. Its wash, **Mask Wash** (`accent-weak` / `accent-weak-light`), fills the selected and hovered quality row, the selected Settings segment and the link box's focus ring. **On-Mask** (`on-accent` / `on-accent-light`) is the ink on orange fills: the logo arrow and the active count.

### Neutral
- **Darkroom Black** (`bg` / `bg-light`, the light table's warm off-white): the window ground, including the left half of New download and the whole Downloads list.
- **Safelight Shadow** (`field` / `field-light`): the title bar and status bar fields, darker than the ground in dark and slightly deeper than it in light.
- **Light Table** (`table` / `table-light`): the Quality column's field on New download; the tint is what divides the two columns.
- **Contact Paper** (`panel` / `panel-light`): resting containers that sit on the ground: download rows and the save-folder row.
- **Easel** (`surface` / `surface-light`): the link box, the Settings dialog and the About popover.
- **Tray** (`surface-2` / `surface-2-light`): secondary buttons, Settings segments, the count at rest, undeveloped sprocket holes, thumbnails before they load, hover behind icon and title-bar buttons, the disabled primary button.
- **Tray Hover** (`hover` / `hover-light`): hover for secondary buttons, segments and selects.
- **Rule** (`border` / `border-light`): the one-pixel rule between quality rows.
- **Strong Rule** (`border-2` / `border-2-light`): outlines of the link box, selects, dialog and popover, the radio ring, the scrollbar thumb.
- **Print White** (`text` / `text-light`, ink on the light table): primary text and the primary button's fill. **Print Hover** (`primary-hover` / `primary-hover-light`) is the primary button's hover: pure white in dark, pure black in light.
- **Proof Gray** (`muted` / `muted-light`): secondary text, icons at rest, unselected tabs, Settings labels, the count at rest.
- **Rebate Tan** (`faint` / `faint-light`): placeholders, notes, chips, the status bar, the edge code, the empty-state icon, disabled text.

### Status
- **Fixer Green** (`ok` / `ok-light`): Done text and check, a finished strip, the ready status dot.
- **Amber Toner** (`warn` / `warn-light`): the status dot while yt-dlp is starting, updating or couldn't check.
- **Safelight Red** (`err` / `err-light`): error messages, a failed strip, the failed-setup dot, a preview that couldn't load.

### Film
- **Film Base** (`film` / `film-light`): the preview frame, the 3px frame around list thumbnails and the empty-state film tile. Near-black in dark, ink in light: film is always dark.
- **Sprocket** (`hole` / `hole-light`): the holes punched in the film rebate (the preview and the empty-state tile), so they read as the ground showing through.

### Named Rules
**The Negative-Mask Rule.** Orange means selection, progress, focus or the active count, plus the two marks this world names: the logo tile and the QUALITY label. It never colors headings, body text, resting icons or large fills; the Mask Wash is the largest area it covers.

**The Print White Rule.** The one primary action (Download) is filled with the text color and labeled in the ground color: print white on darkroom black, ink on paper on the light table. It is never orange and never a status color.

**The Developer Colors Rule.** Fixer Green, Amber Toner and Safelight Red color only status text, icons, dots and the strip of a finished or failed download. They never fill a button or a container.

**The Film Stays Dark Rule.** Film is dark in both themes. Type printed on film uses the dark theme's ink values (Print White, Proof Gray, Rebate Tan, Safelight Red) whatever the window theme.

**The Paired Tokens Rule.** Every color exists in both themes under one name, and text tokens keep at least 4.5:1 on every ground they sit on.

## Typography

**Body Font:** Segoe UI Variable Text (with Segoe UI, system-ui, sans-serif)
**Label/Mono Font:** Bahnschrift SemiCondensed (with Bahnschrift, Segoe UI, sans-serif)

**Character:** Segoe UI Variable is Windows' own voice for words, so the app belongs on the desktop. Bahnschrift, a DIN-style face set semi-condensed, is the edge-print voice: the numbers and codes a lab prints along a film's rebate. Both ship with Windows.

### Hierarchy
- **Figure** (Bahnschrift 600, 20px, 22px, 0.02em, tabular): each download row's one big number, the percent while it runs and the file size when it's done. The largest type in the app.
- **Headline** (600, 15px, 20px): the Settings title and the video title in the preview frame (13px/18px when the frame turns sideways).
- **Action** (700, 15px): the Download button.
- **Title** (600, 13.5px): quality option names; list titles use 600 at the body size.
- **Body** (400, 13px, 18px): the default, including the link box.
- **Control** (500, 12.5px): secondary buttons, selects, the Clear finished link; tabs are 500 at body size.
- **Body small** (400, 12px, 16px): option descriptions, the channel line, download status, notes, About, the status bar.
- **Label** (Bahnschrift 600, 12px, 0.14em, uppercase): QUALITY, THEME, TEXT AND PAGE SIZE.
- **Chip** (Bahnschrift 600, 12px, 0.08em, uppercase): MKV, MP4, "1080P · MP4".
- **Edge code** (Bahnschrift 400, 12px, 14px, 0.06em, tabular): the preview's "ID · resolution · length" line, kept in its original case because video IDs are case-sensitive; the version button, About values and the status bar's right side use the same face at 0.04–0.05em.
- **Count** (Bahnschrift 600, 11px, 16px, 0.04em, tabular): the Downloads History tab count.

### Named Rules
**The Two Voices Rule.** Words are Segoe UI Variable; anything printed on film or counted is Bahnschrift semi-condensed with tabular figures. Uppercase belongs only to the Bahnschrift voice, and never to an ID.

**The One Big Number Rule.** The 20px Figure is the only type above 15px, and each download row has exactly one. Everywhere else hierarchy comes from weight and the three ink values.

## Layout

A fixed desktop frame at 1040×760: a 40px title bar field on top, a 32px status bar field at the bottom, and the active tab between them.

- **New download** is a two-column grid of equal halves, padded 24px top and bottom and 32px at the sides, with 16px between rows and 64px between columns. The left column (link box, preview frame, save folder, Download) sits on the darkroom ground; the right (Quality) on the light-table field. The field edge falls exactly at 50%, which is the only divider. The page order stays link → quality → folder → Download so Tab follows the steps.
- **Downloads History** is one full-width list padded 32px at the sides, 8px between rows, the Clear finished link above it at the right. Each row is thumbnail, info, then a fixed 176px action column, so every row's strip and big number end on the same vertical line.
- **Fixed heights:** 48px for Download and quality rows; 44px for the link box and save-folder row; 34px Settings segments; 32px buttons and icon buttons; 30px selects; 28px title-bar buttons.
- **Rhythm:** 4px between segments, 8px and 12px inside rows, 14px insets that echo the sprocket pitch, 16px between stacked blocks, 22px inside the Settings dialog, 24px and 32px page padding.
- **Text and page size** (100/125/150/175%) zooms the whole page. When the page is shorter than 520px (150% and 175%), the preview frame turns sideways: a 112px thumbnail beside the title, the edge code on its own full-width line below, allowed to wrap rather than lose the ID, so the folder row and Download stay in the first viewport. When the page is narrower than 720px (175%), the logo tile alone carries the brand and Resume, Retry and Show in folder become 32px icon buttons in a 72px action column. Narrower than 580px (175% in a window near its minimum), the Downloads History tab reads "Downloads" and the title bar's gaps tighten to 12px, so the window buttons stay on screen.
- The status bar is three columns with equal sides, so "Developed by Khryz" stays exactly centered.

## Elevation & Depth

Flat at rest. Depth comes from fields and tones: the darker field for the bars, the light-table tint for the Quality column, Contact Paper and Easel for containers, and the film base, which is always the darkest thing on screen. Only layers that float above the window cast a shadow: the Settings dialog (over a dimmed backdrop) and the About popover.

### Shadow Vocabulary
- **Floating** (`box-shadow: 0 14px 36px rgba(0, 0, 0, .5)` dark, `0 14px 36px rgba(26, 23, 20, .16)` light): the Settings dialog and About popover only.
- **Backdrop** (`rgba(7, 6, 5, .6)`): behind the modal Settings dialog, in both themes.

### Named Rules
**The Fields Not Lines Rule.** Regions are divided by tinted fields, never by borders. One-pixel rules belong inside a list (between quality rows) and around fields and floating layers.

**The Flat-At-Rest Rule.** Nothing casts a shadow while it sits in the page; only the dialog and the popover float.

## Shapes

Tight, nearly square corners, like cut film and trimmed prints: 4px for fields, rows, the primary button, the dialog and the popover; 3px for buttons, selects and the count; 2px for film (the preview frame, list thumbnail frames, the empty-state tile); 1px for the picture inside the preview frame. Only radio buttons and status dots are round. Rules and outlines are one pixel; the radio ring is 1.5px. Quality rows are square at rest and take 4px corners only when selected. Thumbnails are 16:9 crops (112×63 sideways, 96×54 in the list inside a 3px film frame).

The sprocket hole is the recurring silhouette: a 9×6px rounded rectangle (1.5px corners) repeated every 14px, drawn as a mask over a color so the same holes form the preview's rebate, the empty-state tile and every download's edge strip.

**The Square Darkroom Rule.** No corner is larger than 4px. Pills, soft cards and large radii belong to another world.

## Components

Exact and tactile: square-cut controls, quiet hover tints, and motion only where film develops.

### Buttons
- **Shape:** 3px corners; the primary button 4px.
- **Primary (Download):** full column width, 48px, Print White fill with Darkroom Black Action type and a leading download icon. Hover goes to Print Hover. Disabled is Tray with Rebate Tan until there's a valid link and yt-dlp is ready.
- **Secondary (Paste, Change, Resume, Retry, Show in folder):** 32px, Tray fill, no border, Control type, optional 14px leading icon; hover Tray Hover.
- **Icon button (Pause, Cancel, Delete, Clear link, dialog close):** 32px square, no fill, Proof Gray icon; hover adds Tray and Print White.
- **Text link (Clear finished):** Proof Gray Control type, no underline; hover Print White.
- **Focus:** a 2px orange outline, 2px offset, on every control (inset on tabs).

### Chips
- **Style:** no fill and no border: Rebate Tan Chip type, uppercase, never wraps; right-aligned in quality rows, beside the title in download rows.
- **Count:** the Downloads History tab count is a 3px-corner Tray tile in Proof Gray Count type; while something downloads it turns orange with On-Mask figures.

### Cards / Containers
- **Download row:** Contact Paper, 4px corners, 12px padding (14px on the left), 16px between thumbnail, info and actions.
- **Save-folder row:** Contact Paper, 4px, 44px, folder icon in Proof Gray, the path in Print White truncating.
- **Settings dialog:** 460px Easel with a Strong Rule outline, 4px corners, 22px padding and gaps, Floating shadow over the Backdrop; Theme and Text and page size as segmented rows.
- **About popover:** 264px Easel, Strong Rule outline, 4px, 16px padding, Floating shadow; a 32px logo tile, then versions as a two-column list (Rebate Tan names, Bahnschrift values) and "Developed by Khryz".

### Inputs / Fields
- **Link box:** 44px Easel with a Strong Rule outline and 4px corners, a Rebate Tan link icon, Body text, an orange caret, Rebate Tan placeholder. Paste sits inside on the right and swaps for a clear icon button once there's text.
- **Focus:** the outline turns orange and a 3px Mask Wash ring surrounds the field.
- **Selects (Custom row):** 30px Tray with a Strong Rule outline, 3px corners, Control type; hover Tray Hover.
- **Segmented control (Settings):** equal 34px segments, 4px gap, Tray at rest with Proof Gray labels; hover Tray Hover; selected takes the Mask Wash, an orange one-pixel outline and Print White.

### Navigation
- **Title bar:** a 40px Safelight Shadow field. Left: the 20px orange logo tile (4px corners, On-Mask download arrow) and "YT Downloader" at 600. Then the tabs, then Settings and the version button (28px, Proof Gray, Tray on hover), then Windows 11 caption buttons drawn by the page: 46px wide, full height, Tray on hover, Close turning Windows' own red with white on hover.
- **Tabs:** text tabs across the title bar's full height, 20px apart, Proof Gray at 500; hover Print White; selected Print White with a 2px orange underline. Arrow keys move between them.

### Quality option
A 48px row on the light-table field, square, divided from the next row by a one-pixel Rule: a 16px Strong Rule radio ring, a Title over a Body-small description, and a chip at the right. Hover lays the Mask Wash. Selected: an orange one-pixel outline, Mask Wash, 4px corners, the rule above it hidden, and the radio becomes a 5px orange ring around a light-table center. The QUALITY label above is orange Label type with a compact Video / Audio switch at the right, and a Rebate Tan note under them ("All options include best audio" or "Audio only, no video"). The switch is the Settings segmented control made small: 26px segments on a Tray track with 2px padding and 3px corners; the chosen side takes the Mask Wash, an orange one-pixel outline and Print White. Video shows the six video rows, Audio shows five audio rows in the same shell (MP3, AAC, WAV, FLAC, WMA, with the file type as the chip). The Custom row keeps the shell with two selects beside the label and wraps them under it when the column is narrow.

### Preview frame (signature)
The video as one frame on a strip of film: a Film Base frame with 2px corners, a 16px rebate of Sprocket holes across the top, the 16:9 picture inset 14px, the edge code printed below in Rebate Tan, then the title in Print White (Headline, two lines at most) and the channel in Proof Gray. A preview that fails hides the picture and edge code and shows its message in Safelight Red.

### Edge strip (signature)
A download's progress is an 8px row of sprocket holes on the 14px pitch, masked over Tray so the undeveloped holes read as the row's own tone. The fill underneath develops orange as bytes arrive and eases its width over 0.5s (`cubic-bezier(.16, 1, .3, 1)`); under reduced motion it jumps to the same state. While merging or paused the fill drops to half opacity. The strip stays in every state: fully developed in Fixer Green when done, stopped in Safelight Red where a download failed, undeveloped when canceled. Beside it, at the row's right edge, the Figure shows the percent while it runs and the file size when it's done.

**The Whole Holes Rule.** The strip's width is rounded down to the 14px pitch and its fill snaps to the pitch, so a hole is either developed or not; never a sliver.

### Status bar
A 32px Safelight Shadow field in Rebate Tan Body small. Left: a 7px status dot (Fixer Green, Amber Toner or Safelight Red) and "yt-dlp **version** · up to date", the version in Proof Gray Bahnschrift. Center: "Developed by Khryz" in Proof Gray. Right: ffmpeg version and check time in Bahnschrift. Long text truncates.

### Empty state
A 64×44 Film Base tile with sprocket rows top and bottom around a Rebate Tan download arrow, then "No downloads yet" at 600 and one Proof Gray line.

## Do's and Don'ts

### Do:
- **Do** keep orange for selection, progress, focus, the active count, the logo tile and the QUALITY label.
- **Do** make the one primary action print white on darkroom black (ink on paper in light), 48px, 4px corners.
- **Do** divide regions with the field and table tints; keep one-pixel rules for list rows, fields and floating layers.
- **Do** set edge codes, chips, labels, counts, versions and the big number in Bahnschrift semi-condensed with tabular figures; set words in Segoe UI Variable.
- **Do** draw progress as the edge strip: whole sprocket holes on the 14px pitch, orange while running, Fixer Green when done, Safelight Red where it stopped.
- **Do** keep film dark in both themes and print on it with the dark theme's inks.
- **Do** define every color for both themes under one name, with text at 4.5:1 or better.
- **Do** check every screen at 100% and 175% text and page size, and in both themes.

### Don't:
- **Don't** make it look like an ad-site downloader: no big green DOWNLOAD buttons, badges or flashing banners.
- **Don't** go neon: no glows, color gradients, RGB accents or glassy blur. The hard 50% edge that paints the two New download fields is a field edge, not a gradient.
- **Don't** bring back the graphite-and-blue look: no cool grays, no blue accent.
- **Don't** fill buttons or containers with Fixer Green, Amber Toner or Safelight Red; they color status only.
- **Don't** round anything past 4px except radios and status dots.
- **Don't** draw progress as a continuous bar or let a strip end on a partial hole.
- **Don't** put shadows on anything that sits in the page; only the dialog and the popover float.
- **Don't** uppercase a video ID or any other case-sensitive code.

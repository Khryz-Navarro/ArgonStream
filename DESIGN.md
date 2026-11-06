---
name: YT Downloader
description: A quiet Windows 11 app for turning a YouTube link into a video file.
colors:
  bg: "#0E0E10"
  panel: "#141416"
  surface: "#1A1A1D"
  surface-2: "#222226"
  hover: "#2A2A2F"
  border: "#26262B"
  border-2: "#34343B"
  text: "#EDEDF0"
  muted: "#A0A0AA"
  faint: "#8A8A94"
  accent: "#7B93FF"
  accent-weak: "rgba(123, 147, 255, .12)"
  ok: "#3ECF8E"
  warn: "#F5B54A"
  err: "#FF6B6B"
  primary-hover: "#FFFFFF"
  bg-light: "#F7F7F8"
  panel-light: "#FFFFFF"
  surface-light: "#FFFFFF"
  surface-2-light: "#EFEFF2"
  hover-light: "#E5E5EA"
  border-light: "#E3E3E8"
  border-2-light: "#D0D0D7"
  text-light: "#17171A"
  muted-light: "#55555F"
  faint-light: "#6B6B75"
  accent-light: "#4059E8"
  accent-weak-light: "rgba(64, 89, 232, .08)"
  ok-light: "#17804F"
  warn-light: "#C27803"
  err-light: "#D3262C"
  primary-hover-light: "#000000"
typography:
  headline:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "15px"
    fontWeight: 600
    lineHeight: "20px"
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
  body-small:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: "16px"
  control:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "12.5px"
    fontWeight: 500
    lineHeight: "18px"
  action:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "14px"
    fontWeight: 600
    lineHeight: "18px"
  label:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "11px"
    fontWeight: 600
    lineHeight: "14px"
    letterSpacing: "0.07em"
  tag:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "11px"
    fontWeight: 600
    lineHeight: "14px"
    letterSpacing: "0.03em"
rounded:
  chip: "5px"
  sm: "6px"
  md: "7px"
  group: "8px"
  lg: "10px"
  xl: "12px"
  dialog: "14px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "24px"
  gutter: "32px"
components:
  button-primary:
    backgroundColor: "{colors.text}"
    textColor: "{colors.bg}"
    typography: "{typography.action}"
    rounded: "{rounded.lg}"
    height: "44px"
  button-primary-hover:
    backgroundColor: "{colors.primary-hover}"
  button-primary-disabled:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.faint}"
  button-secondary:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.text}"
    typography: "{typography.control}"
    rounded: "{rounded.md}"
    padding: "0 12px"
    height: "32px"
  button-secondary-hover:
    backgroundColor: "{colors.hover}"
  button-icon:
    textColor: "{colors.muted}"
    rounded: "{rounded.md}"
    size: "32px"
  button-icon-hover:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.text}"
  input-link:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    typography: "{typography.body}"
    rounded: "{rounded.lg}"
    padding: "0 6px 0 14px"
    height: "44px"
  tab:
    textColor: "{colors.muted}"
    rounded: "{rounded.sm}"
    padding: "0 12px"
    height: "26px"
  tab-selected:
    backgroundColor: "{colors.surface-2}"
    textColor: "{colors.text}"
  quality-option:
    backgroundColor: "{colors.panel}"
    textColor: "{colors.text}"
    rounded: "{rounded.lg}"
    padding: "0 14px"
    height: "46px"
  quality-option-selected:
    backgroundColor: "{colors.accent-weak}"
  chip:
    textColor: "{colors.muted}"
    typography: "{typography.tag}"
    rounded: "{rounded.chip}"
    padding: "2px 6px"
  download-item:
    backgroundColor: "{colors.panel}"
    rounded: "{rounded.xl}"
    padding: "12px"
  dialog:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    rounded: "{rounded.dialog}"
    padding: "20px"
    width: "460px"
  status-bar:
    textColor: "{colors.faint}"
    typography: "{typography.body-small}"
    padding: "0 16px"
    height: "32px"
---

# Design System: YT Downloader

## Overview

**Creative North Star: "The Quiet Utility"**

YT Downloader is a tool that gets out of the way. It opens to calm, near-black layers (or their soft
off-white counterparts), with small, steady Segoe UI type and one blue that appears only where something
is chosen, moving, or focused. Nothing competes with the link box and the quality choice; the window
reads as a native Windows 11 app, not a web page and certainly not a download website.

The feel is **calm, precise, compact and friendly**. Calm comes from low contrast between surfaces and
color that always means something. Precision comes from fixed control heights, tabular numbers and crisp
1px borders. Compactness comes from a 13px body and dense, evenly spaced rows. Friendliness comes from
plain words and soft corners: every control is gently rounded and nothing jumps.

Two looks are explicitly rejected: the **ad-site downloader** (big green DOWNLOAD buttons, badges,
flashing banners) and the **neon gamer** (glows, gradients, RGB accents, glassy blur).

**Key Characteristics:**
- Dark-first, with a complete light theme; "Desktop" follows Windows.
- Depth from stepped surface tones and 1px hairlines, not shadows.
- One accent, Soft Cobalt, reserved for meaning.
- The single primary action is the text color inverted, not the accent.
- Small type (13px body, nothing above 15px); hierarchy from weight and tone.
- Soft, steady controls: 7–12px corners, quiet hover tints, motion only for progress.

## Colors

A near-neutral graphite palette with a single soft blue accent and three status colors, defined twice
(dark by default, light under `prefers-color-scheme: light`) under the same token names.

### Primary
- **Soft Cobalt** (`accent`; `accent-light` in the light theme): the only accent. It marks the selected
  quality option (border plus the Soft Cobalt Wash `accent-weak` tint), the radio dot, progress bar fills,
  focus rings, and the Downloads count while something is downloading.

### Neutral
- **Near-Black Canvas** (`bg` / `bg-light`, a cool off-white): the window background.
- **Graphite Panel** (`panel` / `panel-light`): resting rows and cards, such as quality options, the save
  folder row, download items and the tab group.
- **Raised Graphite** (`surface` / `surface-light`): the link box, preview card, Settings dialog and About
  popover.
- **Control Gray** (`surface-2` / `surface-2-light`): secondary buttons, selected tabs and segments, the
  progress track, thumbnails before they load.
- **Hover Gray** (`hover` / `hover-light`): the hover state of secondary buttons and selects.
- **Hairline** (`border` / `border-light`): 1px dividers and resting card outlines.
- **Strong Hairline** (`border-2` / `border-2-light`): outlines of fields, buttons, chips, dialogs and the
  radio ring.
- **Soft White / Ink** (`text` / `text-light`): primary text, and the fill of the primary button.
- **Quiet Gray** (`muted` / `muted-light`): secondary text, icons, unselected tabs, labels.
- **Whisper Gray** (`faint` / `faint-light`): placeholders, notes, the status bar, disabled text.

### Status
- **Mint Done** (`ok` / `ok-light`): "Done" text and check, the yt-dlp-ready dot.
- **Amber Wait** (`warn` / `warn-light`): the status dot while yt-dlp is starting, updating or couldn't
  check for updates.
- **Coral Error** (`err` / `err-light`): error messages and the failed-setup dot.

### Named Rules
**The Cobalt Means Something Rule.** Soft Cobalt appears only for selection, progress, focus and the
active-download count. Never use it for decoration, headings, icons at rest, or large fills; its Soft
Cobalt Wash tint (`accent-weak`) is the largest area it may cover.

**The Inverted Primary Rule.** The one primary action on a screen (Download) is filled with the text color
and labeled in the background color, so it is white on dark and black on light. It is never Soft Cobalt
and never a status color. There is at most one per screen.

**The Status Words Rule.** Mint, Amber and Coral color only status text, icons and dots. They never fill
a button or a card.

**The Both Themes Rule.** Every new color is defined for dark and light under the same token name, and
text tokens keep at least 4.5:1 contrast on `bg` and `surface-2` in both.

## Typography

**Body Font:** Segoe UI Variable Text (with Segoe UI, system-ui, sans-serif)

**Character:** Windows' own UI face at small sizes, so the app reads as part of the system. One family
does everything; weight (400, 500, 600) and tone (text, muted, faint) carry the hierarchy, not size.

### Hierarchy
- **Headline** (600, 15px, 20px): dialog titles, such as "Settings". The largest type in the app.
- **Title** (600, 13.5px, 18px): quality option names; video titles in the preview and download list use
  the same weight at 13px.
- **Body** (400, 13px, 18px): the default for everything, including the link box text.
- **Body small** (400, 12px, 16px): secondary lines such as option descriptions, preview channel and
  resolution, download status, notes, the status bar.
- **Control** (500, 12.5px): labels on secondary buttons, selects and text links.
- **Action** (600, 14px): the primary Download button.
- **Label** (600, 11px, 0.07em, uppercase): section labels such as QUALITY, THEME, TEXT AND PAGE SIZE.
- **Tag** (600, 11px, 0.03em): chips with short codes such as MKV, MP4, "1080p · MP4".

### Named Rules
**The 15px Ceiling Rule.** Nothing in the app is larger than 15px. Emphasis comes from weight 600 and
from the text tone against muted and faint, never from bigger type.

**The Lined-Up Numbers Rule.** Numbers that change or get compared (percentages, versions, sizes in the
About list, the version button) use tabular figures.

## Layout

A fixed desktop frame: a 40px title bar on top, a 32px status bar at the bottom, and the active tab's
content between them. The window opens at 1040×760 and can't go below 900×600.

- **New download** is a two-column grid of equal halves with a 32px gutter and 24px/32px page padding:
  link box, preview, save folder and Download stack on the left with 16px between them; Quality fills the
  right. The order in the page stays link → quality → folder → Download, so Tab follows the steps.
- **Downloads** is one full-width list, padded 32px at the sides, with 10px between items and the "Clear
  finished" link above it.
- **Rows are fixed heights:** 44px for the link box, save folder and Download button; 46px for quality
  options; 32px for buttons and icon buttons; 26px for tabs; 28px for title-bar buttons.
- **Spacing rhythm:** 4px between quality options, 8–12px inside rows, 16px between stacked blocks, 24px
  and 32px for page padding and gutters.
- **Text and page size** (100%, 125%, 150%, 175%) zooms the whole page, title bar included. Every screen
  must stay usable at each step; long titles and paths truncate with an ellipsis instead of wrapping.
- The status bar is three columns, two equal sides and the center, so "Developed by Khryz" is exactly
  centered.

## Elevation & Depth

Flat at rest. Depth comes from stepping through the surface tones (`bg` → `panel` → `surface` →
`surface-2`) and from 1px hairlines. Shadows belong to layers that float above the window: the Settings
dialog and the About popover use the one shadow token, over a dimmed backdrop for the dialog. A faint lift
on hover is allowed for cards and rows (download items, quality options), much softer than the floating
shadow; no token exists yet, so define one in the stylesheet the first time it is used.

### Shadow Vocabulary
- **Floating** (`box-shadow: 0 12px 32px rgba(0, 0, 0, .45)` dark, `0 12px 32px rgba(17, 17, 26, .14)`
  light): the Settings dialog and the About popover only.
- **Backdrop** (`rgba(0, 0, 0, .45)`): behind the modal Settings dialog.

### Named Rules
**The Flat-At-Rest Rule.** Nothing casts a shadow while it sits still. Hover may add a faint lift to a
card or row; only floating layers get the full shadow.

## Shapes

Gently rounded throughout, with corners that grow with the size of the thing: 5px chips, 6px tabs and
thumbnails, 7px buttons and selects, 8–9px segmented groups, 10px fields, rows and the primary button,
12px download cards and the About popover, 14px for the Settings dialog. Radio buttons, status dots and
the empty-state ring are full circles. Borders are always 1px. Thumbnails are 16:9 crops (112×63 in the
preview, 96×54 in the list) with `object-fit: cover`.

**The Nested Corners Rule.** An element inside another always has a smaller radius than its container (a
7px button inside a 10px row inside a 12px card), so corners stay concentric.

## Components

Soft and steady: gently rounded, quiet hover tints, and no motion beyond what shows progress.

### Buttons
- **Shape:** gently rounded (7px); the primary button 10px.
- **Primary (Download):** full column width, 44px tall, inverted fill (text color on background color),
  Action type with a leading download icon. Hover goes to pure white (dark) or pure black (light).
  Disabled is Control Gray with Whisper Gray text until there's a valid link and yt-dlp is ready.
- **Secondary (Paste, Change, Resume, Retry, Show in folder):** 32px, Control Gray fill, Strong Hairline
  border, Control type, optional 14px leading icon. Hover: Hover Gray.
- **Icon button (Pause, Cancel, Delete, Clear link, dialog close):** 32px square, no fill, Quiet Gray icon;
  hover adds Control Gray and the text color.
- **Text link (Clear finished):** Quiet Gray Control type with no underline; hover turns it to the text
  color.
- **Focus:** a 2px Soft Cobalt outline with a 2px offset on every control.

### Chips
- **Style:** transparent with a 1px Strong Hairline border, 5px corners, 2px/6px padding, Quiet Gray Tag
  type, never wraps. Short uppercase codes: MKV, MP4, "1080p · MP4".
- **Count badge:** the Downloads tab count is a pill of Strong Hairline; while a download is active it
  turns Soft Cobalt with background-colored text.

### Cards / Containers
- **Corner Style:** 10px (preview, rows), 12px (download items, About).
- **Background:** Graphite Panel for list items and rows; Raised Graphite for the preview and floating
  layers.
- **Shadow Strategy:** flat at rest; see Elevation & Depth.
- **Border:** 1px Hairline.
- **Internal Padding:** 10–12px; download items 12px with 14px between thumbnail, text and buttons.

### Inputs / Fields
- **Link box:** 44px, Raised Graphite fill, Strong Hairline border, 10px corners, a link icon in Whisper
  Gray, Body text, Whisper Gray placeholder. The Paste button sits inside on the right and swaps for a
  clear (×) icon button once there's text.
- **Focus:** the border turns Soft Cobalt and a 3px Soft Cobalt Wash ring appears around the field.
- **Selects (Custom row):** 30px, Control Gray fill, Strong Hairline border, 7px corners, Control type;
  hover Hover Gray.
- **Save folder row:** the same 44px shape on Graphite Panel with a Hairline border; the path truncates.

### Navigation
- **Tabs (title bar):** a segmented group on Graphite Panel with a Hairline border, 8px corners and 3px
  padding. Each tab is 26px with 6px corners in Quiet Gray, weight 500; the selected tab gets a Control
  Gray fill and the text color. Arrow keys move between tabs.
- **Segmented control (Settings):** the same pattern at 32px, with an optional 14px icon before each label.
- **Title bar:** 40px with a 1px Hairline below. The brand on the left (20px Soft Cobalt logo tile with a
  background-colored download arrow, then "YT Downloader" at 600), the tabs, then the Settings cog and
  version button on the right (28px, Quiet Gray, Control Gray on hover). Window buttons are Windows 11's:
  native in the Electron build, drawn by the page at 46px wide in the Tauri build, where Close turns
  `#C42B1C` with white on hover.

### Quality option (signature)
A 46px row on Graphite Panel with a Hairline border and 10px corners: a 16px radio ring, a Title over a
Body-small description, and a chip on the right. Hover strengthens the border to Strong Hairline. Selected:
Soft Cobalt border, Soft Cobalt Wash fill, and the radio becomes a 5px Soft Cobalt ring around a
background-colored dot. The Custom row keeps the same shell with two selects beside the label; using
either select chooses Custom.

### Download item (signature)
A 12px-corner card: a 96×54 thumbnail, then the title (600, truncated) with its quality chip, a status
line (Body small, Quiet Gray) with the percentage right-aligned in the text color, and a 4px progress bar
(Control Gray track, Soft Cobalt fill, 2px corners). Done shows a Mint check and Mint text; errors are
Coral and may wrap; canceled is Whisper Gray. While merging or paused, the fill drops to 45% opacity.
Actions sit on the right as icon buttons (Pause, Cancel, Delete) or secondary buttons (Resume, Retry,
Show in folder), showing only what applies to the current state.

### Status bar
32px with a 1px Hairline above, Body small in Whisper Gray. Left: a 7px status dot (Mint, Amber or Coral)
and "yt-dlp **version** · up to date", with the version in Quiet Gray at 500. Center: "Developed by Khryz"
in Quiet Gray. Right: ffmpeg version and check time. Long text truncates.

### Motion
The only animation is the progress fill easing its width over 0.2s linear; it is turned off under
`prefers-reduced-motion`. Hover and selection changes are instant.

## Do's and Don'ts

### Do:
- **Do** use Segoe UI Variable Text at 13px for body copy, and stay at or below 15px everywhere.
- **Do** keep Soft Cobalt for selection, progress, focus and the active count only.
- **Do** make the one primary action an inverted fill (text color on background color), 44px, 10px
  corners.
- **Do** build depth from `bg` → `panel` → `surface` → `surface-2` and 1px hairlines; save the floating
  shadow for dialogs and popovers.
- **Do** define every new color for both themes under one token name, with text at 4.5:1 or better.
- **Do** keep corners concentric: 7px controls inside 10px rows inside 12px cards.
- **Do** check new screens at 175% text and page size and in both themes.
- **Do** keep window chrome looking like Windows 11 (caption buttons, Segoe UI, theme following Windows).

### Don't:
- **Don't** make it look like an ad-site downloader: no big green DOWNLOAD buttons, badges, starbursts or
  flashing banners.
- **Don't** go neon: no glows, gradients, RGB accents or glassy blur.
- **Don't** use Mint, Amber or Coral on buttons or backgrounds; they color status only.
- **Don't** put shadows on resting cards and rows; at most a faint lift on hover.
- **Don't** add animation beyond progress; state changes are instant.
- **Don't** introduce a second accent color or a second font family.

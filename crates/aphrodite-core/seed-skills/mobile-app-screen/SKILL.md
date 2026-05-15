---
name: mobile-app-screen
description: Mobile-first app screens — phone-framed composition, iOS-or-Android safe-area conventions, single-handed thumb-reach hierarchy, bottom-anchored primary actions.
version: 1.0.0
tags:
  - mobile_app
  - mobile
  - ios
  - android
  - touch-ui
related_skills:
  - asset-standards
agent_created: false
default: false
---

# Mobile app screen

## When this skill applies

Intent mentions: mobile app, iOS app, Android app, native phone app, mobile screen. The deliverable is a phone-shaped composition (390 × 844 px territory) framed within the page, demonstrating the screen's hierarchy and touch affordances — not a responsive web design.

## Workflow

1. **Phone frame.** Wrap the screen content in a 390 × 844 px container (iPhone 14 baseline). Rounded corners 48-52 px (iPhone). Status bar at the top with time (current), signal/wifi/battery glyphs. Home-indicator bar at the bottom 5 px tall, centred, 134 px wide.

2. **Safe areas.** Content sits inside safe-area margins: 16-20 px horizontal, 44 px below status bar, 34 px above home indicator. Never put primary touch targets in the safe-area zone.

3. **Touch target minimum 44 × 44 px.** Apple HIG. Tap surfaces (buttons, list items, switches) must hit this floor even if the visual appears smaller. Padding compensates.

4. **Single-handed thumb-reach hierarchy.** The bottom third of the screen is the most reachable for right-handed thumbs. Primary actions belong there (FAB, bottom tab bar, primary CTA). The top third is for navigation context (back, title, more) — operations the user *commits* to with a deliberate reach.

5. **Layout pattern:**
   - Status bar (built into the frame)
   - Top app bar (44-56 px tall): back chevron + title + optional trailing action icon
   - Screen content area (scrollable)
   - Bottom tab bar (49 px tall) OR bottom primary CTA — never both
   - Optional sheet / modal overlay (slides up from bottom, dismissible by drag handle)

6. **Type scale (iOS Dynamic Type baseline):**
   - Large title 34 px
   - Title 28 px
   - Headline 17 px semibold
   - Body 17 px regular
   - Callout 16 px
   - Subhead 15 px
   - Footnote 13 px
   - Caption 12 px / 11 px
   - System font: SF Pro / Inter (web fallback)

7. **Icons.** Lucide at 24 × 24 px in tab bars, 20 × 20 px inline with list items. SF Symbols-equivalent in spirit. Tab bar icons are LABELED (icon + 10 px caption beneath).

## Do / Don't

- DO show the phone frame with rounded corners, status bar, home indicator.
- DO use a bottom tab bar (3-5 tabs) or a bottom-anchored primary CTA, not both.
- DO put primary actions in the bottom third (thumb reach).
- DO show a back chevron in the top-left when navigation depth > root.
- DON'T centre titles unless explicitly iOS-style (Apple HIG allows it; Android Material doesn't).
- DON'T use a hamburger menu — modern iOS / Android conventions favour bottom tab bars.
- DON'T over-use modal dialogs. Slide-up sheets are the preferred non-blocking alternative.
- DON'T put primary CTAs in the top-right corner of a mobile screen — that's the *furthest* reach for right-handed thumbs.

## Reference fragments worth lifting

- Phone frame: 390 × 844 px, border-radius 48 px, background `--colors-neutral-900` (the "device chassis")
- Inner screen: 358 × 802 px (16-px horizontal padding × 2), background `--colors-background-primary`
- Status bar: 44 px tall, content right-aligned at 13 px medium
- Tab bar: 49 px tall, 5 icons max, icons 24 px + label 10 px, active state in accent
- FAB: 56 × 56 px floating action button, bottom-right, 16-24 px from edges
- Touch target hit areas: minimum 44 × 44 px (visible smaller is OK; padded hit area meets the floor)

## Cross-references

- Persona: `naoto-fukasawa` is the natural pair (without-thought design, gesture-driven, type recedes).
- Wiki: future entries for Apple Music, Headspace, Linear Mobile worth adding.

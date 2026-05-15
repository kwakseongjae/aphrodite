---
name: "mushin-timer"
version: "0.1"
description: "Single-purpose meditation timer — swipe up to begin, nothing else"
colors:
  primary:
    "50": "#f7f5f2"
    "100": "#eae5de"
    "200": "#d4cbc0"
    "300": "#b5a899"
    "400": "#978775"
    "500": "#8b7d6b"
    "600": "#7a6d5c"
    "700": "#635849"
    "800": "#524a3e"
    "900": "#3d3832"
  neutral:
    "0": "#ffffff"
    "50": "#fafaf8"
    "100": "#f0efec"
    "200": "#e0ddd8"
    "300": "#c4c0b9"
    "400": "#9a968f"
    "500": "#716d67"
    "600": "#55524d"
    "700": "#3f3d39"
    "800": "#2a2926"
    "900": "#1a1918"
    "1000": "#000000"
typography:
  display:
    family: "Inter"
    weight: 300
  body:
    family: "Inter"
    weight: 400
spacing:
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "6": "24px"
  "8": "32px"
  "12": "48px"
  "16": "64px"
  "24": "96px"
rounded:
  sm: "4px"
  md: "8px"
  lg: "16px"
  xl: "24px"
  pill: "9999px"
metadata:
  variants:
    light:
      description: "Quiet morning — paper and unglazed clay"
      tokens:
        colors.background.primary: "#fafaf8"
        colors.text.primary: "#2a2926"
    dark:
      description: "Evening room — dimmed, close"
      tokens:
        colors.background.primary: "#1a1918"
        colors.text.primary: "#e0ddd8"
    brand-a:
      description: "Warm clay — terracotta dust on white"
      tokens:
        colors.background.primary: "#f5f0ea"
        colors.text.primary: "#2e231a"
    brand-b:
      description: "Stone garden — cool mineral grey"
      tokens:
        colors.background.primary: "#f0f0ec"
        colors.text.primary: "#1e1e1c"

---

# Overview

This is a single-surface timer. One gesture — swipe up — and the session begins. There is no onboarding, no mode selector, no profile screen. The screen is the object. Like the wall-mounted CD player, the gesture is borrowed from something the hand already knows: pulling a window shade, lifting a cover, drawing a curtain open. Swipe up. The timer starts. That is the entire interaction contract.

The colour palette is built from the grey-brown family of unglazed pottery, raw linen, old plaster. Not warm enough to read as "sand" or "beige" in a catalogue sense — just a warm neutral that breathes. The primary at 500 is a muted umber, present in the swipe affordance and the elapsed-time ring. It withdraws everywhere else.

Four variants exist because meditation happens at different hours and under different moods, but every variant maintains the same principle: the background and text should feel like looking at a wall you have known for years. Contrast ratios are verified at 7:1 or above — well beyond AA — because this screen may be viewed in low light with unfocused eyes.

# Colors

The primary hue family sits around 30° on the colour wheel — the ochre-umber range — deliberately avoiding the blue and purple associations common to meditation apps. This is not a "calming" colour in the marketing sense. It is the colour of things that do not call attention to themselves: clay, cork, cardboard, stone.

In light mode the background is a near-white with a faint yellow-grey undertone (#fafaf8). Text is warm charcoal (#2a2926). The contrast ratio is approximately 13.5:1. In dark mode the relationship inverts: #1a1918 background with #e0ddd8 text, yielding roughly 12.3:1. Both exceed AAA requirements because the timer will be read peripherally, during practice, without focused gaze.

Brand-a introduces a terracotta-warm variant (#f5f0ea / #2e231a, ~12.8:1). Brand-b shifts toward mineral coolness (#f0f0ec / #1e1e1c, ~14.6:1). The four variants are not themes in the "personalisation" sense — they are the same object in four different rooms, under four different lights.

# Typography

Inter at light weight (300) for display, regular (400) for body. The decision to avoid display or bold weights is deliberate: this is a timer, not a poster. The session duration reads at 48px light — large enough to register in peripheral vision, light enough to feel insubstantial, like watching minutes pass on a clock face across a room.

Body text sits at 15px regular. The handful of secondary labels — "swipe up to begin," the elapsed indicator — are 13px regular in neutral-400, intentionally low-contrast. They are present for first-time orientation and then should disappear from awareness entirely. If a user can read these labels effortlessly after the third session, they are too prominent.

No emoji in the timer display. No iconographic decoration. A single Lucide icon — the chevron-up — appears in the swipe affordance zone, and even this should feel like part of the surface texture rather than a control label.

# Layout

The screen is divided into three vertical zones. The upper third is rest: generous empty space with only the session duration centered in light type. The middle third is the breathing zone — where the eye naturally settles during practice — left deliberately empty. The lower third contains the swipe affordance: a subtle pill-shaped track with a small chevron, positioned at the natural resting point of the thumb.

Safe areas are observed: 16px horizontal margins, 44px below the status bar, 34px above the home indicator. No top app bar exists. No tab bar. No navigation. The status bar and home indicator — system chrome — are the only structural elements the user sees beyond the timer itself. This is a full-screen experience by default, because a meditation timer that shows you the time and signal strength has already failed at its purpose.

The swipe-up gesture area occupies the bottom 120px of the screen. The thumb rests here naturally. The affordance is a thin horizontal pill (120 × 4px, neutral-300) with a small chevron-up icon centered above it. As the user swipes upward, the pill track elongates to fill the full screen width and fades — the curtain opens. The session begins when the gesture completes.

# Elevation & Depth

There is no elevation system. No shadows. No floating cards. The screen is a single flat surface — like a plaster wall, like the face of an analog clock. Depth would imply layers, and layers imply complexity that this timer does not possess.

The swipe affordance uses opacity rather than shadow to communicate interactivity. The pill track sits at 40% opacity in its resting state, rising to 100% during the gesture. The chevron pulses once — a single slow fade from 30% to 50% and back — when the screen first appears, then remains still. Stillness is the default. Motion is the exception.

During an active session, the screen darkens by approximately 3% over the first 30 seconds — barely perceptible, like the room settling. When the session completes, a single soft tone plays and the screen gently lightens back to its resting state. No animation choreography. No confetti.

# Shapes

Two radii are used in the entire interface: 4px for the swipe pill track and 24px for the device frame itself. Everything else is orthogonal. The screen content sits flush within the device — no internal rounded rectangles, no card containers, no pill-shaped buttons.

The swipe pill track at the bottom is 120 × 4px with a 4px radius — effectively a short, softly-capped line. It is the minimum viable affordance: present enough to invite the thumb, restrained enough to ignore once the gesture is learned. After three uses, it should be invisible to conscious attention.

The device frame — visible in design specifications but not in the user's experience — is an iPhone 14 profile at 390 × 844px with 48px corner radii. The frame exists in documentation only. The user's experience is edge-to-edge surface.

# Components

The timer display is a single text node: session duration in Inter Light at 48px, centered vertically in the upper 60% of the screen. Default display is "10:00" — a reasonable default session. No picker, no scroll wheel. The duration is set outside this screen, in a settings view accessed by a single small gear icon that appears only when the user long-presses the timer text. This is the CD player's hidden volume knob: present for those who need it, invisible to everyone else.

The swipe affordance zone is not a button. It is a gesture area. Visually it is the pill track and chevron. Functionally it is the entire bottom 120px of the screen. The touch target exceeds the 44px minimum by a wide margin because the gesture should feel generous, not precise. You do not aim at it. You simply swipe upward, as you would to unlock a door.

The elapsed-time ring is a thin circular stroke (1.5px, neutral-200) that fills with primary-400 as time passes. It surrounds the duration text at a radius of approximately 80px. It is the only dynamic element during a session. Its movement is slow enough to read as stillness — the minute hand of a clock, not a loading bar. When the ring completes, the session ends.

The completion state replaces the duration text with a single word in 17px regular: "done." The ring is full. After five seconds, the screen returns to its resting state with the duration reset. No dismissal gesture required. The timer is ready for the next session.

# Do's and Don'ts

Do let the gesture borrow from existing muscle memory. Swiping up to start is pulling a shade, lifting a cover. The hand already knows this. Use what it knows.

Do allow the interface to disappear after the first three sessions. Every element — the chevron, the pill track, the duration text — should recede into the user's peripheral awareness. If they can describe the interface in detail, it is too loud.

Do choose the lightest weight that reads. Inter Light at 48px for the timer. Inter Regular at 15px for body. Heavier weights announce themselves; this timer should not announce.

Do leave the middle of the screen empty. This is where the eye rests during practice. Filling it with progress bars, breathing animations, or inspirational quotes turns a tool into a performance.

Don't add a breathing animation. The user is breathing already. An animated circle that tells them when to inhale is not a meditation aid — it is a metronome with opinions.

Don't use blue or purple because they signal "meditation app" in the marketplace. The purpose of this timer is not to look like a meditation app. It is to time meditation. The colour of that action is clay, not lavender.

Don't show the current time, battery level, or notification count during a session. These are the exact things the user sat down to step away from. If the system allows it, hide the status bar entirely during active sessions.

Don't congratulate the user. No streak counters, no session history, no share buttons. The session is its own purpose. Adding gamification to a meditation timer is adding a second engine to a rowboat — it moves, but you have stopped rowing, and that was the whole point.
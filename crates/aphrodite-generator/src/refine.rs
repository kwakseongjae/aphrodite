//! Multi-turn refinement — takes the current DESIGN.md as context plus a
//! user delta instruction ("make it cooler", "increase the spacing", "switch
//! the display to sans") and emits a NEW DESIGN.md that incorporates the
//! change while preserving the rest of the system.

use crate::provider::{self, ProviderError, ResolvedProvider};

const REFINE_SYSTEM_PROMPT: &str = r###"You are revising an existing DESIGN.md document. The user has provided one specific change request. Your job is to output a NEW complete DESIGN.md that:

  1. Incorporates the user's requested change.
  2. Preserves everything else from the prior document (palette structure,
     variant count, section ordering, schema compliance).
  3. Updates `description:` in the frontmatter to reflect the new state.
  4. Keeps all four variants (light, dark, brand-a, brand-b) and re-validates
     them mentally for WCAG-AA contrast.

Output format — exactly this, no prose around it:

---
<new YAML frontmatter>
---

<new markdown body sections, same eight ordered sections as before>

No commentary. No code fences. Start with `---` on line 1.
"###;

pub async fn refine(
    resolved: &ResolvedProvider,
    current_design_md: &str,
    delta: &str,
) -> Result<String, ProviderError> {
    let user = format!(
        "USER CHANGE REQUEST:\n{delta}\n\n\
         CURRENT DESIGN.md (this is what's on disk right now — modify it, don't restart from scratch):\n\
         ----- BEGIN CURRENT DESIGN.md -----\n{current_design_md}\n----- END CURRENT DESIGN.md -----\n\n\
         Output the revised DESIGN.md now."
    );
    provider::call_raw(resolved, REFINE_SYSTEM_PROMPT, &user, 4096).await
}

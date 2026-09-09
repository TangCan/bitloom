# Process: one story → one commit

Lesson from Epic 18 (and earlier retros): bundling multiple stories into a single commit weakens per-story attribution and review.

**Rule for future epics:** land each story as its own commit (message ties to the story id/title). Do not squash an epic into one commit unless a human explicitly requests it.

## Diff purity (epic-26-retro-item-59)

Contract / story commits must stay scoped to the story. Do **not** fold unrelated example reformats, tool-script churn (e.g. `render_skill.py`), or drive-by formatting into the same commit as AC/contract changes. Historical noise: `e30e6ca` (`examples/bundle_vec_skel`), `b758832` (`render_skill.py`). Hygiene or tooling edits → separate commit or omit.

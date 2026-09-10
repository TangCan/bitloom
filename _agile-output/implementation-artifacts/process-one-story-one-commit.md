# Process: one story → one commit

Lesson from Epic 18 (and earlier retros): bundling multiple stories into a single commit weakens per-story attribution and review.

**Rule for future epics:** land each story as its own commit (message ties to the story id/title). Do not squash an epic into one commit unless a human explicitly requests it.

## Diff purity (epic-26-retro-item-59)

Contract / story commits must stay scoped to the story. Do **not** fold unrelated example reformats, tool-script churn (e.g. `render_skill.py`), or drive-by formatting into the same commit as AC/contract changes. Historical noise: `e30e6ca` (`examples/bundle_vec_skel`), `b758832` (`render_skill.py`). Hygiene or tooling edits → separate commit or omit.

## NFR14 / closeout commit subject (epic-42–47 retro items 109/113/116/120/124/128)

Gate and closeout (and implementation) commits must put a story id in the **subject** so `git_evidence.py` can attribute them. Prefer ASCII short forms:

- `Story N.1` / `Story N.M` (e.g. `Story 47.2`)
- or the sprint short id (`47.1`, `47.2`, `47.3`)

When running evidence scripts, pass short ids via `--stories` rather than full Chinese sprint keys. Subjects that only use Chinese epic titles with no `Story N.M` / short id produce empty attribution.

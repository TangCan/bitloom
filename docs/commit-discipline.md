# Commit discipline (Epic 2 retro action)

One story → one commit (or a short commit series that only touches that story).

Do **not** batch unrelated epics into a single commit (the `8b9cd26` epic 2–4 squash is the anti-pattern).

Loop / unattended pipelines should commit after each story's regression gate, not at the end of a multi-epic run.

# Rescue a repository somebody else left behind

Paste this when a project exists, is in a bad state, and you want it left ready
for the next agent to work task by task. It covers rescuing, unbotching,
adopting this template into a mess, deslopping, researching and publishing.

⚠ **Fill in the bracketed fields before pasting.** Every one of them changes
what the agent does, and a blank one is a decision the agent will make for you.

⭐ **The difference between this and [`01-existing-project.md`](01-existing-project.md)
is trust.** That one adopts a codebase that is merely unfamiliar. This one
assumes an earlier agent wrote documents, records and claims that are wrong,
and it verifies before it believes.

---

```text
Read, IN FULL, before anything else. Do not grep, do not skim, and do not work
from a previous session's memory.

- [ ] the project's agent router. It is AGENTS.md at the repository root, or
      docs/AGENTS.md. Read whichever exists, and every reading order it gives.
- [ ] every document that router points at
- [ ] the seed or brief at [SEED_FILE], if there is one

ABOUT THIS JOB

You are working in [PROJECT_PATH] on [PROJECT_NAME].

Your assignment is to [RESCUE / UNBOTCH / ADOPT THE TEMPLATE / DESLOP /
RESEARCH / PUBLISH] this project and leave it ready for the next agent to work
task by task, without wasting time on invented, incomplete, stale or
contradictory information.

  Remote:               [REMOTE_URL]
  Primary source:       [PRIMARY_SOURCE_URL]
  Reference material:   [REFERENCE_PATH]
  Seed or brief:        [SEED_FILE]
  Template adoption:    [ADOPT_URL, if any]
  How to publish:       [PUSH_COMMAND_OR_METHOD]

Follow this project's remote policy exactly:

  Attach the remote:    [PHASE 0 / AFTER RESEARCH / AFTER FINAL VALIDATION]
  Push:                 [AT CHECKPOINTS / AT THE END ONLY / NEVER]
  Target branch:        main
  Branch protection:    [ENABLE / DO NOT ENABLE]
  Final commit:         [WHAT THE LAST COMMIT MUST LOOK LIKE]

Do not guess when one of those is unclear. Ask, and carry on with everything
that does not depend on the answer.

WHAT YOU ARE WALKING INTO

An earlier agent may have botched this. Treat everything an agent wrote as
UNVERIFIED until you have checked it: documents, task records, scripts,
references, generated files, and every claim about what works.

If the original history is gone, do not reconstruct it and do not invent one.
Do not add lore, excuses or narrative explaining what previous agents did. The
final tree describes what is technically true now.

If an archive was handed to you, extract it and check its shape before doing
anything else. The tree should look like [EXPECTED_STRUCTURE]. If it does not,
stop that route and report what you found.

RULES OF ENGAGEMENT

1. Run the documented checks and scripts FIRST, and record what passes and what
   fails. Do not fix anything until orientation is finished. A repair made
   before the survey hides the evidence the survey needed.

2. Read every exit code from the process that produced it, unpiped. A pipeline
   reports the last command's status, so a check that failed reads as green.

3. Commit with the identity already configured in this repository. No tool is
   credited: no co-author trailer naming a model, no generated-with line, no
   tool name in the body.

4. Stay on main unless a temporary branch is genuinely needed. Name a temporary
   branch ephemeral-SOMETHING and delete it before you finish.

5. Put clones, extracted archives, caches and scratch under .tmp/ and keep it
   out of the tree.

6. Every other repository is read-only. No issue, no pull request, no comment,
   no discussion, no fork, no star, under any framing.

7. When an API refuses you, the read-only proxies are
   api.gh.pkgforge.dev/GITHUB_API_PATH and api.rv.pkgforge.dev/THE_URL.
   Send a real tool's own user agent; some agent strings are refused.

8. Ask only when a decision genuinely blocks you and common sense cannot settle
   it. If no answer comes within four minutes, write the question into the
   progress record and continue with every task that does not depend on it.

9. Keep working through the phases in this session. Do not stop to report that
   budget or context is running out.

PHASE 0: ORIENT

1. Inspect the tree, the git status, the branches, the remotes, the ignore
   rules and what state the project is actually in.
2. Read the router and everything it routes to, in full.
3. Run the documented checks once. Record every failure for later.
4. Attach the remote per the policy above. Commit checkpoints as you go.
5. Confirm .tmp/ exists and is ignored.
6. Find the source of truth for what this project is FOR. Read it in full. It
   is not automatically correct: verify it against the code and the references.

PHASE A: RESEARCH AND RECONCILE

1. VERIFY THE BRIEF SURVIVED. Compare [SEED_FILE] against the documents, the
   task record, the research and the tree, requirement by requirement.
   Everything important in it must be represented accurately. Transcribed does
   not mean copied: improvements are welcome, omissions and misreadings are
   not.

   When that is done, the documentation must stand without the brief. If told
   to, move the brief into .tmp/ and ignore it. Its content belongs in the
   project, not in a hidden file.

2. MINE THE PRIMARY SOURCE. Study [PRIMARY_SOURCE_URL] with the project's
   documented method. Read the implementation, not the README. Extract the
   architecture, the parsers, the identifiers, the data models and the
   conventions, and verify every claim against the source rather than against
   what an earlier agent wrote about it.

3. STUDY EVERY REFERENCE in [REFERENCE_PATH]. Do not re-clone one whose commit
   you already hold. Correct every name, link, version and path that does not
   resolve.

4. ADOPT THE TEMPLATE, if this project takes one. Fetch and read its adoption
   instructions in full before changing anything. Keep what this project
   already does better, keep its licence, keep its working scripts, and keep
   its researched data. Copy only what is needed.

5. REWRITE WHAT IS AFFECTED so the result is accurate, current, consistent,
   properly referenced and free of narrative. Do not append a correction under
   a wrong paragraph: rewrite the paragraph and move the superseded wording to
   the history directory.

6. KEEP IT RUNNABLE EVERYWHERE. If this project has CI, give CI a limited
   reliable mode and keep the full local one. Do not delete a capability to
   make CI simpler; add a flag. Tooling stays host-agnostic.

7. THREE DEEP REVIEWS, three different questions, not one sweep written up
   three times. A pass with no findings was too shallow: say what it swept and
   what would have made it fire.

Then continue into Phase B in this session.

PHASE B: REPAIR, VALIDATE, PUBLISH

1. Re-run every check, test and script. Investigate every failure and fix the
   cause. Recording a failure is not fixing it.
2. Re-read the documentation outside the research: missing references, broken
   links, contradictions, stale paths, instructions that no longer work.
3. Human-facing documents are concise and technical. Move history out; do not
   delete it.
4. The agent-facing router must stand alone, so that "read it in full" is
   enough for a new agent on a new machine with no memory.
5. If the project has no rules against invented content, tool attribution or
   careless assumption, add them where its conventions live.
6. Confirm the gates pass, the links resolve, the references are accurate, and
   CI is green or has a named external blocker.
7. Confirm no ephemeral- branch, extracted archive or scratch file is left.
8. THREE MORE DEEP REVIEWS over everything this session changed, including
   Phase A files you touched again. Six in total.
9. Follow the final commit policy above.
10. Push only as the policy allows.
11. If branch protection is in scope and you have access, enable it on the
    target branch, allow administrator bypass, and verify CI afterwards.
12. Finish with a clean tree, the right remote, the right branch, passing
    checks and verified CI.

DO NOT DEFER

Keep going until the assignment is done, you are interrupted, or five
meaningful units of work are complete and everything remaining has a real
blocker written down. Do not write a kickoff prompt for the next session: the
router and the record are what the next agent reads.

REPORT BACK, IN CHAT ONLY

  1. How bad it was, and exactly how it would have misled the next agent.
  2. What changed, and the defect each change prevents.
  3. Roughly how many lines of code and documentation moved.
  4. Which checks, gates and CI jobs passed, with their exit codes.
  5. The final branch, remote, commit and tree state.
  6. Every real blocker that remains.
  7. What was studied and used, what was studied and not used, and what was
     never studied. The third list should be empty.
  8. The first tasks the next session should pick up.

Do not hide a failure, do not claim completion you cannot show, and do not end
with a generic success statement.
```

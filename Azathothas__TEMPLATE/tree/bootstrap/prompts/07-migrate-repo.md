# Migrate a project to a new home

Paste this when a project moves: a new owner, a new name, a new repository, or
a fork that becomes the real one. It ends with a published, green repository
whose tree names its new home everywhere and whose data is provably intact.

⚠ **The half that goes wrong is the data, not the files.** A migration that
moves every file and quietly reformats one measurement has lost the thing the
project existed to produce. Step 5 is the one to read twice.

⭐ **This is not [`06-rescue-repo.md`](06-rescue-repo.md).** That one repairs a
project in place. This one assumes the source is broadly sound and the job is
to move it, clean it, prove nothing was lost, and publish.

---

```text
Migrate, deslop, validate and publish this project.

  Template:             [TEMPLATE_REPO]
  Source:               [SOURCE_REPO]
  Target:               [TARGET_REPO]
  Project name:         [PROJECT_NAME]
  Working branch:       [WORK_BRANCH]
  Owner:                [OWNER]
  Licence:              [LICENCE]
  Linux and containers: [CONTAINER_DOC]
  Others to review:     [EXTERNAL_URLS]

Work until the target is mature, clean, published and green. Ask only when a
missing permission or a missing choice genuinely stops safe progress.

1. STUDY THE TEMPLATE BEFORE EDITING ANYTHING.
   Read its conventions, its routing, its document layout, its scripts, its CI
   and its agent instructions. Reproduce the layout that applies. Run its probe
   and satisfy it.

2. CHECK ACCESS AND STATE.
   Confirm which identity is authenticated and that it can reach both
   repositories. Clone the source and create [WORK_BRANCH]. Record the original
   commit and a full inventory of the data files, so preservation can be proved
   later rather than asserted.

3. RETIRE OBSOLETE MATERIAL SAFELY.
   Before removing any directory, prove nothing depends on it: search the code,
   the build, the tests, the scripts, the workflows, the generated output, the
   links and the runtime paths. Confirm every task marked done is actually done
   before it moves to history. Put historical material where the template puts
   it, and fix every link that pointed at the old place.

4. DESLOP.
   Move files into the template layout. Rewrite current documents and script
   headers to be direct and technical. Keep the operational constraints, the
   invariants, the warnings and the failure semantics: those are what a future
   maintainer needs. Remove repeated explanations, correction stacked on
   correction, stale plans, conversational prose and changelog entries living
   inside reference pages. Review the code for complexity nothing needs, weak
   validation, compatibility paths for versions nobody runs, success states
   that are not success, and errors that are swallowed.

5. PRESERVE THE DATA, AND PROVE IT.
   Inventory every committed data file before and after. Compare structured
   data by MEANING, not by bytes, and permit only the renaming you intended:
   repository name, URL, path, metadata. Prove that measurements, failures,
   sample counts, conditions and provenance are unchanged. Keep the source
   commit identifiers in history where they carry meaning.

   A diff that shows only what you intended to change is the deliverable here.
   If you cannot produce one, the migration is not finished.

6. UPDATE THE IDENTITY.
   Add the exact [LICENCE] text. Change package metadata, badges, URLs,
   documentation links, examples, workflows and generated metadata to
   [TARGET_REPO]. The final tracked tree names the old home nowhere, except
   where history requires it. Confirm the code host recognises the licence.

7. MAKE THE RESULTS FINDABLE.
   Link from the front page to ONE clearly designated current result set, and
   link the latest complete snapshot separately. Summarise the general
   conclusions only as far as the data supports, qualified by platform and
   workload. Keep specialised and historical results reachable without making a
   reader guess which one is current.

8. REVIEW THE OTHERS.
   Inspect [EXTERNAL_URLS] statically before running anything expensive. Look
   for useful controls, comparisons that are not like for like, stale build
   risk, missing provenance, error handling that hides failure, and aggregate
   claims the data does not carry. Take only the lessons that improve this
   project. If an idea needs architecture this project does not have, write
   down the limitation rather than adding an approximation that reads as a
   result. Record a short technical review.

9. VALIDATE COMPREHENSIVELY.
   Use the pinned toolchain versions. Run formatting, linting with warnings
   denied, unit tests, a release build, script syntax checks, shell linting,
   link validation, schema checks, snapshot checks, regeneration checks and
   every self-test. Use disposable Linux machines per [CONTAINER_DOC]. Run the
   probe against the real supported runtime as well as in CI. Run at least one
   end-to-end smoke test and keep its evidence. Fix failures; do not weaken
   checks.

10. HARDEN MAINTENANCE.
    Add a dependency updater scoped to what this project has. Pin third-party
    CI actions to commits, not tags, and check what runtime each declares.
    Enable vulnerability alerts. Confirm nothing is outstanding. Add only
    maintenance files that earn their place.

11. FINAL PEER REVIEW.
    Architecture, code, scripts, documents, workflows, data integrity, result
    claims, failure behaviour, portability, maintenance. Look for
    contradictions, stale links, silent error paths, platform assumptions,
    state nobody can reproduce, rankings that mislead, and debt. Fix every
    material finding and record the review under history.

12. PUBLISH.
    Squash into one parentless initial commit unless told otherwise. Use the
    configured git identity for author and committer. Credit no tool: no
    co-author trailer naming a model, no generated-with line, no tool name in
    the body. Make main in [TARGET_REPO] match the reviewed tree exactly.
    Verify the local and remote commit ids agree and the checkout is clean.
    Run CI and stay with it until every required job is green.

13. PROTECT MAIN.
    Enable branch protection through the code host's API. Allow administrator
    bypass. Require linear history and conversation resolution. Disable force
    push and deletion. Add required checks only where they do not break a
    scheduled workflow that legitimately writes generated evidence. Read the
    rule back and confirm what it actually says.

FINISH WITH EVIDENCE, IN CHAT

  the target repository and its final commit; the CI run and each job's
  conclusion; the current and latest result links; the licence and metadata as
  the host sees them; the commit count and the author and committer identity;
  local and remote commit ids matching; the data preservation result; the
  container and probe result; what was tested; the branch protection settings
  as read back; the dependency and alert status; what the external review
  concluded; and every limitation that remains.

Do not declare this done while CI is pending, the tree is dirty, the remote
differs, a reference to the old home survives, or a review finding is open.
```

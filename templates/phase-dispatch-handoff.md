# Phase N (<name>) — dispatch guide for the implementing session

You are building **<one-sentence scope>**. The authoritative spec is
[`NN-<phase>.md`](NN-<phase>.md) — **read it in full before doing anything.** This document is
the map, the ground rules, and the state of the world you're inheriting.

## 0. State of the world

- **What shipped:** Phase N-1 is committed and pushed — commit `<sha>` on `<branch>`. It
  delivered <dirs/files>, validated by <exact commands / observed results>.
- **⚠️ What is torn down / not running:** <e.g. the cloud node was destroyed to stop billing>.
  Your first step is: <exact commands to bring it back, with time estimate>.
- **⚠️ The binding constraint:** <RAM / cost / an API limit> — and what to do if you hit it.
- **Already exists — REUSE, don't reinvent:** <paths to reference implementations and why>.

## 1. Ground rules

- Every phase ends green (`just check` / `just ci`). Commit per increment.
- Don't widen scope; record follow-ups in <backlog file> instead.
- Anything that spends money or touches prod: stop and ask.

## 2. Plan of increments

| # | Increment | Done when (a command that passes, not an adjective) |
|---|-----------|------|
| 1 | | |

## 3. Known gotchas (do not rediscover)

-

## 4. When you finish

Write `PHASE<N>-COMPLETION-HANDOFF.md`: what shipped (commits), what deviated from the spec and
why, what's left running/torn down, measured numbers, and the gotchas you hit.

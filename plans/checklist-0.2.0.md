# Zerum v0.2.0 implementation checklist

Prompt-aligned execution checklist for `prompts/prompt-0.2.0.md` and `plans/plan-0.2.0.md`.

Status legend:
- `[ ]` not started
- `[-]` in progress
- `[x]` done

---

## 0) Scope guardrails (must stay true)

- [x] No external checker execution
- [x] No SARIF reporter
- [x] No LLM behavior
- [x] No plugin/dynamic loading/WASM
- [x] No autofix
- [x] CLI remains deterministic-first and educational

---

## 1) Milestone 0 — planning + tutorial skeleton

- [x] Confirm final CLI scope:
  - [x] `check`
  - [x] `explain`
  - [x] `list-checks`
  - [x] `init`
- [x] Confirm output scope:
  - [x] `human`
  - [x] `json`
- [x] Create tutorial files:
  - [x] `docs/tutorial/04-writing-checks.md`
  - [x] `docs/tutorial/05-explain-mode-and-configuration.md`
  - [x] `docs/tutorial/06-config-profiles.md` (v0.4.0)
- [x] README documents v0.2.0+ architecture and install
- [x] Milestone reviews: `reviews/milestone-3` through `milestone-6-v0.2.0.md`

---

## 2) Milestone 1 — rule engine + metadata

### Core structures

- [x] Add/extend `CheckMetadata` in core:
  - [x] `id`
  - [x] `name`
  - [x] `category`
  - [x] `severity`
  - [x] `safe_fixable`
  - [x] `examples`
- [x] Update `Check` trait to expose metadata cleanly
- [x] Ensure registry stores metadata for all checks

### CLI wiring

- [x] `list-checks` prints metadata-rich output
- [x] `explain` reads metadata + guidance from unified path

### Config behavior

- [x] Per-check enable/disable honored consistently
- [x] Per-check severity override implemented

### Tests

- [x] Metadata integrity test (`registry_metadata_is_complete_and_aligned`)
- [x] Rule ID uniqueness + format test
- [x] Config override tests for severity (`tests/config_override_tests.rs`)

### Milestone output

- [x] Covered in milestone 3–6 review notes

---

## 3) Milestone 2 — parser wrapper boundary

- [x] Introduce parser wrapper/semantic access layer (`SourceModel` in `src/parser/source_model.rs`)
- [x] Keep rustpython internals encapsulated behind local abstraction
- [ ] Provide semantic iterators/helpers:
  - [ ] functions
  - [ ] classes
  - [ ] imports
  - [ ] comments
- [ ] Update existing checks to consume wrapper instead of raw parser internals where practical

### Tests

- [ ] Wrapper unit tests for each semantic accessor
- [ ] UTF-8/line-column regression tests remain green
- [ ] Fixture coverage for comments/import extraction

### Milestone output

- [ ] Milestone 2 review note

---

## 4) Milestone 3 — readability + warning expansions

### Readability (`ZR001`–`ZR015`)

- [x] ZR001 LongFunction
- [x] ZR002 TooManyArguments
- [x] ZR003 DeepNesting
- [x] ZR004 ComplexBooleanExpression
- [x] ZR005 UnclearVariableName
- [x] ZR006 MagicNumber
- [x] ZR007 MissingModuleDocstring
- [x] ZR008 MissingFunctionDocstring
- [x] ZR009 CommentedOutCode
- [x] ZR010 TODOWithoutContext
- [x] ZR011 NarratorDocstring
- [x] ZR012 BoilerplateDocstring
- [x] ZR013 StepComment
- [x] ZR014 NarratorComment
- [x] ZR015 ObviousComment

### Warning (`ZR401`–`ZR415`)

- [x] ZR401 BroadExcept
- [x] ZR402 EmptyExcept
- [x] ZR403 BareExcept
- [x] ZR404 MutableDefaultArgument
- [x] ZR405 PrintDebugging
- [x] ZR406 AssertProduction
- [x] ZR407 DangerousEvalExec
- [x] ZR408 SilentExceptionSwallowing
- [x] ZR409 GlobalMutableState
- [x] ZR410 AmbiguousNoneReturn
- [x] ZR411 BlanketExcept
- [x] ZR412 QueryInLoop
- [x] ZR413 SilentFallback
- [x] ZR414 LengthComparison
- [x] ZR415 SortForTopK

### Explain + tests (for this milestone’s rules)

- [x] Explain-mode content exists for each implemented rule
- [ ] Per rule: 1 positive + 1 negative test minimum
- [ ] Fixture scenarios for readability/warning categories
- [x] Snapshot updates (`human`, `json`)

### Milestone output

- [x] Milestone 3 review note (`reviews/milestone-3-v0.2.0.md`)

---

## 5) Milestone 4 — consistency + refactor expansions

### Consistency (`ZR101`–`ZR110`)

- [x] ZR101 InconsistentFunctionNaming
- [x] ZR102 InconsistentClassNaming
- [x] ZR103 InconsistentConstantNaming
- [x] ZR104 InconsistentImportStyle
- [x] ZR105 MixedQuoteStyle
- [x] ZR106 InconsistentTestNaming
- [x] ZR107 InconsistentPrivatePrefix
- [x] ZR108 DuplicateNamingPattern
- [x] ZR109 MixedCollectionStyle
- [x] ZR110 MixedReturnStyle

### Refactor (`ZR301`–`ZR315`)

- [x] ZR301 DuplicateBranchBody
- [x] ZR302 CollapsibleIf
- [x] ZR303 UnnecessaryElseAfterReturn
- [x] ZR304 RedundantBooleanComparison
- [x] ZR305 SimplifiableIfExpression
- [x] ZR306 RepeatedLiteral
- [x] ZR307 LongParameterList
- [x] ZR308 ExtractableCondition
- [x] ZR309 RepeatedTryExcept
- [x] ZR310 IdentityPassthrough
- [x] ZR311 IdentityMap
- [x] ZR312 RejectNone
- [x] ZR313 FilterNone
- [x] ZR314 ManualStringJoin
- [x] ZR315 SortThenReverse

### Explain + tests

- [x] Explain-mode entries for all new rules
- [ ] Naming/style fixture set added
- [ ] Refactor-opportunity fixture set added
- [x] Snapshot updates (`human`, `json`)

### Milestone output

- [x] Milestone 4 review note (`reviews/milestone-4-v0.2.0.md`)

---

## 6) Milestone 5 — design + AI slop expansions

### Design (`ZR201`–`ZR210`)

- [x] ZR201 GodClass
- [x] ZR202 TooManyInstanceVariables
- [x] ZR203 TooManyPublicMethods
- [x] ZR204 FeatureEnvy
- [x] ZR205 DataClassWithoutBehavior
- [x] ZR206 CircularImport
- [x] ZR207 ForbiddenArchitectureImport
- [x] ZR208 LayerViolation
- [x] ZR209 ServiceObjectExplosion
- [x] ZR210 ExcessiveIndirection

### AI (`ZR501`–`ZR510`) — deterministic only

- [x] ZR501 PlaceholderGeneratedCode
- [x] ZR502 GeneratedCommentPattern
- [x] ZR503 ExcessiveNarration
- [x] ZR504 GenericExceptionMessage
- [x] ZR505 BoilerplateParameterDocs
- [x] ZR506 EmptyWrapperFunction
- [x] ZR507 GeneratedDeadBranch
- [x] ZR508 DefensiveOverengineering
- [x] ZR509 ExcessiveAbstraction
- [x] ZR510 GenericUtilityExplosion

### Explain + tests

- [x] Explain entries include false positives/tradeoffs
- [ ] Architecture fixture coverage for design rules
- [ ] AI slop fixture coverage for deterministic patterns
- [x] Snapshot updates (`human`, `json`)

### Milestone output

- [x] Milestone 5 review note (`reviews/milestone-5-v0.2.0.md`)

---

## 7) Milestone 6 — release hardening

### Docs + tutorials

- [x] Update `README.md` to reflect 0.2.0 scope
- [x] Ensure tutorial chapters 00–05 are accurate and complete
- [x] Ensure each tutorial includes:
  - [x] design decisions
  - [x] alternatives
  - [x] complexity/tradeoffs
  - [x] limitations
  - [x] implementation details
- [x] Draft `docs/RELEASE_v0.2.0.md`
- [x] Update `CHANGELOG.md`

### Quality gates

- [x] `cargo fmt` clean (if formatting policy requires)
- [x] `cargo clippy -- -D warnings` clean
- [x] `cargo test` clean
- [x] `cargo publish --dry-run` clean (`docs/RELEASING.md`; use clean git tree for real publish)

### Coverage goal

- [x] Measure and document progress toward `>=70%` coverage target (`docs/coverage.md`)

### Milestone output

- [x] Milestone 6 review note (`reviews/milestone-6-v0.2.0.md`)

---

## 8) Global acceptance checklist (v0.2.0)

- [x] Approx. 75 deterministic rules implemented across all target categories
- [x] Every registered rule supports `explain`
- [x] Config enable/disable + severity override stable
- [x] Reporters limited to `human` + `json`
- [x] Unit + fixture + snapshot + reporter tests present and green
- [x] Educational artifacts complete per prompt (tutorial 04–05, release notes)
- [x] No out-of-scope features were introduced

---

## 9) Current open items (priority)

- [x] Add metadata integrity + severity override test coverage
- [x] Implement parser semantic wrapper (`SourceModel`) and migrate checks
- [-] Improve rule quality from broad heuristics to AST-precise implementations
  - [x] ZR010 TODOWithoutContext hardened (owner/context-aware)
  - [x] ZR406 AssertProduction switched to AST stmt detection
  - [x] ZR407 DangerousEvalExec switched to AST call-target detection
  - [x] ZR414 LengthComparison switched to AST compare detection
  - [x] ZR006 MagicNumber switched to AST constant detection
  - [x] Added `catalog_detectors` AST pass for ZR005/009/105/106/205/209/304/301/302/303/305/308/309/310/311/312/313/314/315/402/408/409/410/411/412/413/415/501/502/504/507/508
- [-] Add per-rule positive/negative tests (especially new catalog rules)
  - [x] Added rule precision tests for ZR010/ZR406/ZR407/ZR414
  - [x] Expanded precision tests for ZR009/304/402/408/409/501/507
  - [x] Added `tests/catalog_fixture_tests.rs` category fixtures
- [x] Expand fixtures for consistency/refactor/design/AI categories
- [x] Create tutorial chapters 04 and 05 and milestone review notes
- [x] Draft `docs/RELEASE_v0.2.0.md` and update `CHANGELOG.md` for final release


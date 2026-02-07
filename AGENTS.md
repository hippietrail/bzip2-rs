# Agent Instructions

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

## Technical Context

### bzip2 Block Markers (BCD format)
- **Block marker (π)**: `314159265359` hex = bytes `31 41 59 26 53 59`
- **Stream end marker (√π)**: `177245385090` hex = bytes `17 72 45 38 50 90`
- Markers appear in the **compressed bitstream**, not decompressed data
- Block boundaries are found by scanning for these bit patterns

### Testing & Validation
- Use `seek-table` tool (on PATH) to get ground truth block offsets
- Format: `compressed_bit_offset decompressed_block_size`
- Example: `seek-table tests/samplefiles/sample1.bz2` outputs `32 98696`
- ⚠️ **Limitation**: Current seek-table on PATH doesn't handle multistream bzip2 files - only reports first stream's offsets
- Use `/Users/hippietrail/itty_bitty/` to inspect bitstreams with bit-level precision
- For multistream validation, may need alternative tool or implement custom validation

### Important Notes
- **Monostream vs Multistream**: Focus on monostream files (single bzip2 stream per file)
- Multistream files are concatenated bzip2 files (lower priority)
- Feature branch: `feature/block-offsets` for block offset tracking work

## Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --status in_progress  # Claim work
bd close <id>         # Complete work
bd sync               # Sync with git
```

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds


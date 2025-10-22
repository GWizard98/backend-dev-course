# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Repository status
- Type: Markdown-first content repo (no build/test tooling configured)
- Default branch: main
- Key dirs: `docs/`, `modules/`, `scripts/`

## Commands
- View syllabus: `less docs/outline.md` (or open it in your editor)
- List modules: `ls -1 modules`
- Search content: `rg -n "term"` (if ripgrep installed) or `grep -R "term" .`
- Scaffold a module: `scripts/new-module.sh "Module Title" [slug]`
  - Creates `modules/NN-slug/` with `README.md`, `labs/`, `assessments/`
  - Note: The script uses `find ... -maxdepth`; on macOS/BSD `find`, install GNU findutils (`gfind`) or adjust the script accordingly.

Build/lint/test
- Build: none
- Lint: none
- Tests: none

## Architecture overview
- Curriculum source of truth lives in `docs/outline.md`.
- Each module is a self-contained folder under `modules/NN-name/`:
  - `README.md` (objectives, prerequisites, lecture notes)
  - `labs/` (hands-on exercises)
  - `assessments/` (quizzes, rubrics, specs)
- Module numbering (`NN-...`) is assigned by `scripts/new-module.sh` by counting existing `modules/[0-9][0-9]-*` directories and incrementing.

## Workflow notes
- Keep changes small and focused; propose significant structure/tooling changes via issues before adding new tooling.

## GitHub repository
- Initialize and commit:
  - `git init && git add -A && git commit -m "chore: initial import"`
- Create a new GitHub repo (with GitHub CLI):
  - Public: `gh repo create {{owner}}/{{repo}} --source=. --public --push`
  - Private: `gh repo create {{owner}}/{{repo}} --source=. --private --push`
- Link to an existing GitHub repo:
  - `git branch -M main`
  - `git remote add origin git@github.com:{{owner}}/{{repo}}.git`
  - `git push -u origin main`

## Polyglot modules (language-agnostic)
- This repo is tooling-agnostic. When a module contains runnable code (any language), document its local commands in that module's `README.md`.
- Recommended snippet to include per module:

```md path=null start=null
### Local development
- Build: <command>
- Lint: <command>
- Test all: <command>
- Test single file/case: <command>
- Run app/service: <command>
```

## Later additions (do not assume until agreed)
- Static site generation (e.g., MkDocs or Docusaurus)
- CI for link checking and linting
- Packaging/export for LMS integrations

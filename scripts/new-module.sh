#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: scripts/new-module.sh \"Module Title\" [slug]" >&2
  exit 1
fi

TITLE="$1"
SLUG="${2:-}"

# Compute next module number (portable across BSD/GNU)
COUNT=$(ls -1d modules/[0-9][0-9]-* 2>/dev/null | wc -l | tr -d ' ')
NEXT_NUM=$(printf "%02d" $((COUNT + 1)))

# Default slug from title if not provided
if [[ -z "$SLUG" ]]; then
  SLUG=$(echo "$TITLE" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-+|-+$//g')
fi

DIR="modules/${NEXT_NUM}-${SLUG}"
mkdir -p "$DIR"/labs "$DIR"/assessments

cat > "$DIR"/README.md <<EOF
# Module ${NEXT_NUM}: ${TITLE}

## Objectives
- 

## Prerequisites
- 

## Lecture Notes
- 

## Labs
- 

## Assessments
- 
EOF

echo "Created $DIR"

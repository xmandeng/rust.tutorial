#!/bin/bash
# Create a GitHub release for an existing, already-pushed tag using gh CLI.
#
# Usage:
#   create-release.sh <tag> <title> --notes "<release notes>"
#   create-release.sh <tag> <title> --notes-file <path-to-notes-file>
#
# Uses --verify-tag: fails if the tag does not already exist on the remote,
# so this script never creates a tag as a side effect.
#
# Use --notes-file for any body longer than a few lines — passes the file
# directly to gh, avoiding shell quoting issues with multi-line text.

set -e

TAG="$1"
TITLE="$2"
shift 2 || true

if [ -z "$TAG" ] || [ -z "$TITLE" ] || [ -z "$1" ]; then
    echo "Error: Missing required arguments" >&2
    echo "Usage: $0 <tag> <title> --notes \"<text>\"" >&2
    echo "       $0 <tag> <title> --notes-file <path>" >&2
    exit 1
fi

if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed" >&2
    echo "Install: https://cli.github.com/" >&2
    exit 1
fi

case "$1" in
    --notes)
        NOTES="$2"
        if [ -z "$NOTES" ]; then
            echo "Error: --notes requires a non-empty value" >&2
            exit 1
        fi
        gh release create "$TAG" --verify-tag --title "$TITLE" --notes "$NOTES"
        ;;
    --notes-file)
        NOTES_FILE="$2"
        if [ -z "$NOTES_FILE" ] || [ ! -f "$NOTES_FILE" ]; then
            echo "Error: --notes-file requires a path to an existing file (got: ${NOTES_FILE:-<empty>})" >&2
            exit 1
        fi
        gh release create "$TAG" --verify-tag --title "$TITLE" --notes-file "$NOTES_FILE"
        ;;
    *)
        echo "Error: expected --notes or --notes-file, got: $1" >&2
        exit 1
        ;;
esac

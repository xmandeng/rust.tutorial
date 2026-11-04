#!/bin/bash
# Attach one or more files to a Jira issue

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib.sh
source "$SCRIPT_DIR/lib.sh"

# Arguments
ISSUE_KEY="$1"
shift || true

# Validation
if [ -z "$ISSUE_KEY" ] || [ "$#" -eq 0 ]; then
    echo "Error: Missing required arguments" >&2
    echo "Usage: $0 <issue-key> <file> [file ...]" >&2
    exit 1
fi

# Check environment variables
if [ -z "$ATLASSIAN_SITE_NAME" ] || [ -z "$ATLASSIAN_USER_EMAIL" ] || [ -z "$ATLASSIAN_API_TOKEN" ]; then
    echo "Error: Jira environment variables not set" >&2
    exit 1
fi

# Build curl multipart args, validating each file exists
FILE_ARGS=()
for f in "$@"; do
    if [ ! -f "$f" ]; then
        echo "Error: File not found: $f" >&2
        exit 1
    fi
    FILE_ARGS+=(-F "file=@${f}")
done

# Upload. The attachments endpoint requires the X-Atlassian-Token: no-check
# header; curl sets the multipart/form-data Content-Type (with boundary) from -F.
BODY=$(jira_request POST "${ATLASSIAN_SITE_NAME}/rest/api/2/issue/${ISSUE_KEY}/attachments" \
    -H "X-Atlassian-Token: no-check" \
    "${FILE_ARGS[@]}")
echo "$BODY" | jq '.'

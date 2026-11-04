#!/bin/bash
# Delete a Jira issue

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib.sh
source "$SCRIPT_DIR/lib.sh"

# Arguments
ISSUE_KEY="$1"

# Validation
if [ -z "$ISSUE_KEY" ]; then
    echo "Error: Missing issue key" >&2
    echo "Usage: $0 <issue-key>" >&2
    echo "" >&2
    echo "Example: $0 QUE-123" >&2
    exit 1
fi

# Check environment variables
if [ -z "$ATLASSIAN_SITE_NAME" ] || [ -z "$ATLASSIAN_USER_EMAIL" ] || [ -z "$ATLASSIAN_API_TOKEN" ]; then
    echo "Error: Jira environment variables not set" >&2
    exit 1
fi

# Delete issue. Jira answers 204 with no body on success; any other status
# stops the script here, before the success line below.
jira_request DELETE "${ATLASSIAN_SITE_NAME}/rest/api/2/issue/${ISSUE_KEY}" > /dev/null

echo '{"success": true, "deleted": "'$ISSUE_KEY'"}'

#!/bin/bash
# Delete a link between two Jira issues by link id

set -e

# Arguments
LINK_ID="$1"

# Validation
if [ -z "$LINK_ID" ]; then
    echo "Error: Missing link id" >&2
    echo "Usage: $0 <link-id>" >&2
    echo "" >&2
    echo "Example: $0 10681" >&2
    echo "Link ids are listed in the issue's fields.issuelinks[].id" >&2
    exit 1
fi

# Check environment variables
if [ -z "$ATLASSIAN_SITE_NAME" ] || [ -z "$ATLASSIAN_USER_EMAIL" ] || [ -z "$ATLASSIAN_API_TOKEN" ]; then
    echo "Error: Jira environment variables not set" >&2
    exit 1
fi

# Delete link. Success is HTTP 204 with no body.
STATUS=$(curl -s -o /dev/null -w '%{http_code}' -X DELETE \
    -u "${ATLASSIAN_USER_EMAIL}:${ATLASSIAN_API_TOKEN}" \
    "${ATLASSIAN_SITE_NAME}/rest/api/2/issueLink/${LINK_ID}")

if [ "$STATUS" != "204" ]; then
    echo "Error: link deletion failed with HTTP ${STATUS}" >&2
    exit 1
fi

jq -n --arg id "$LINK_ID" --arg status "$STATUS" \
    '{success: true, deleted_link: $id, http_status: $status}'

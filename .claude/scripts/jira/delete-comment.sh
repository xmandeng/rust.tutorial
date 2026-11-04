#!/bin/bash
# Delete a comment from a Jira issue by comment id

set -e

# Arguments
ISSUE_KEY="$1"
COMMENT_ID="$2"

# Validation
if [ -z "$ISSUE_KEY" ] || [ -z "$COMMENT_ID" ]; then
    echo "Error: Missing required arguments" >&2
    echo "Usage: $0 <issue-key> <comment-id>" >&2
    echo "" >&2
    echo "Example: $0 QUE-122 15687" >&2
    echo "Comment ids are listed in the issue's fields.comment.comments[].id" >&2
    echo "The delete is permanent." >&2
    exit 1
fi

# Check environment variables
if [ -z "$ATLASSIAN_SITE_NAME" ] || [ -z "$ATLASSIAN_USER_EMAIL" ] || [ -z "$ATLASSIAN_API_TOKEN" ]; then
    echo "Error: Jira environment variables not set" >&2
    exit 1
fi

# Delete comment. Success is HTTP 204 with no body. The status code is appended
# on its own line so the body of a rejected delete can be reported.
RESPONSE=$(curl -s -w '\n%{http_code}' -X DELETE \
    -u "${ATLASSIAN_USER_EMAIL}:${ATLASSIAN_API_TOKEN}" \
    "${ATLASSIAN_SITE_NAME}/rest/api/2/issue/${ISSUE_KEY}/comment/${COMMENT_ID}")

STATUS="${RESPONSE##*$'\n'}"
BODY="${RESPONSE%$'\n'*}"

if [ "$STATUS" != "204" ]; then
    echo "Error: comment deletion failed with HTTP ${STATUS}" >&2
    echo "$BODY" >&2
    exit 1
fi

jq -nc --arg issue "$ISSUE_KEY" --arg id "$COMMENT_ID" --arg status "$STATUS" \
    '{success: true, issue: $issue, deleted_comment: $id, http_status: $status}'

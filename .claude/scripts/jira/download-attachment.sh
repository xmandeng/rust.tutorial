#!/bin/bash
# Download a Jira attachment's content by attachment id

set -e

# Arguments
ATTACHMENT_ID="$1"
OUTPUT_PATH="$2"

# Validation
if [ -z "$ATTACHMENT_ID" ] || [ -z "$OUTPUT_PATH" ]; then
    echo "Error: Missing required arguments" >&2
    echo "Usage: $0 <attachment-id> <output-path>" >&2
    echo "" >&2
    echo "Example: $0 10231 /tmp/diagram.png" >&2
    echo "Attachment ids are listed in the issue's fields.attachment[].id" >&2
    exit 1
fi

# Check environment variables
if [ -z "$ATLASSIAN_SITE_NAME" ] || [ -z "$ATLASSIAN_USER_EMAIL" ] || [ -z "$ATLASSIAN_API_TOKEN" ]; then
    echo "Error: Jira environment variables not set" >&2
    exit 1
fi

# Download. The content endpoint replies with a redirect to the media store,
# so follow redirects (-L). Success is HTTP 200 with the file body.
STATUS=$(curl -s -L -o "$OUTPUT_PATH" -w '%{http_code}' -X GET \
    -u "${ATLASSIAN_USER_EMAIL}:${ATLASSIAN_API_TOKEN}" \
    "${ATLASSIAN_SITE_NAME}/rest/api/2/attachment/content/${ATTACHMENT_ID}")

if [ "$STATUS" != "200" ]; then
    rm -f "$OUTPUT_PATH"
    echo "Error: download failed with HTTP ${STATUS}" >&2
    exit 1
fi

SIZE=$(wc -c < "$OUTPUT_PATH")
echo '{"success": true, "attachment_id": "'"$ATTACHMENT_ID"'", "output_path": "'"$OUTPUT_PATH"'", "bytes": '"$SIZE"'}'

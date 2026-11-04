#!/bin/bash
# Move issues to a different issue type (and optionally a new parent) via the
# bulk move endpoint. This is the only mechanism that can change an issue's
# hierarchy level (e.g. Subtask -> Story, Task -> Epic).

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib.sh
source "$SCRIPT_DIR/lib.sh"

# Subcommand: check the status of a previously submitted move task
if [ "$1" = "task-status" ]; then
    TASK_ID="$2"
    if [ -z "$TASK_ID" ]; then
        echo "Error: Missing task ID" >&2
        echo "Usage: $0 task-status <task-id>" >&2
        exit 1
    fi
    BODY=$(jira_request GET "${ATLASSIAN_SITE_NAME}/rest/api/3/task/${TASK_ID}")
    echo "$BODY" | jq '.'
    exit 0
fi

# Arguments
ISSUE_KEYS="$1"          # comma-separated, e.g. QUE-342,QUE-343
TARGET_PROJECT="$2"      # e.g. QUE
TARGET_TYPE_ID="$3"      # numeric issue type ID, e.g. 10003 for Story
TARGET_PARENT="$4"       # optional parent issue key, e.g. QUE-327

# Validation
if [ -z "$ISSUE_KEYS" ] || [ -z "$TARGET_PROJECT" ] || [ -z "$TARGET_TYPE_ID" ]; then
    echo "Error: Missing required arguments" >&2
    echo "Usage: $0 <issue-keys-csv> <target-project> <target-issuetype-id> [target-parent-key]" >&2
    echo "       $0 task-status <task-id>" >&2
    echo "" >&2
    echo "Examples:" >&2
    echo "  Convert to Epic:  $0 QUE-327 QUE 10004" >&2
    echo "  Subtasks->Story:  $0 QUE-342,QUE-343 QUE 10003 QUE-327" >&2
    exit 1
fi

# Check environment variables
if [ -z "$ATLASSIAN_SITE_NAME" ] || [ -z "$ATLASSIAN_USER_EMAIL" ] || [ -z "$ATLASSIAN_API_TOKEN" ]; then
    echo "Error: Jira environment variables not set" >&2
    exit 1
fi

# Comma-separated keys -> JSON array
KEYS_ARRAY=$(echo "$ISSUE_KEYS" | jq -R 'split(",") | map(gsub("^\\s+|\\s+$";""))')

# The mapping key encodes the destination: "<project>,<issueTypeId>" with an
# optional ",<parent ID or key>" third segment when the moved issues need a
# parent in the target hierarchy.
MAPPING_KEY="${TARGET_PROJECT},${TARGET_TYPE_ID}"
if [ -n "$TARGET_PARENT" ]; then
    MAPPING_KEY="${MAPPING_KEY},${TARGET_PARENT}"
fi

# Build payload; statuses and fields are inferred so existing values carry
# over wherever the target type's workflow allows them.
PAYLOAD=$(jq -n \
    --arg key "$MAPPING_KEY" \
    --argjson keys "$KEYS_ARRAY" \
    '{
        sendBulkNotification: false,
        targetToSourcesMapping: {
            ($key): {
                inferClassificationDefaults: true,
                inferFieldDefaults: true,
                inferStatusDefaults: true,
                inferSubtaskTypeDefault: true,
                issueIdsOrKeys: $keys
            }
        }
    }')

# Submit the move
RESPONSE=$(jira_request POST "${ATLASSIAN_SITE_NAME}/rest/api/3/bulk/issues/move" \
    -H "Content-Type: application/json" \
    -d "$PAYLOAD")

TASK_ID=$(echo "$RESPONSE" | jq -r '.taskId // empty')
if [ -z "$TASK_ID" ]; then
    echo "Error: Bulk move was not accepted" >&2
    echo "$RESPONSE" | jq '.' >&2
    exit 1
fi

# Poll the async task until it finishes
for _ in $(seq 1 30); do
    TASK=$(jira_request GET "${ATLASSIAN_SITE_NAME}/rest/api/3/task/${TASK_ID}")
    STATUS=$(echo "$TASK" | jq -r '.status // empty')
    case "$STATUS" in
        COMPLETE)
            echo "$TASK" | jq '{taskId: .id, status: .status, result: .result}'
            exit 0
            ;;
        FAILED|CANCELLED|DEAD)
            echo "Error: Move task ${TASK_ID} ended with status ${STATUS}" >&2
            echo "$TASK" | jq '.' >&2
            exit 1
            ;;
    esac
    sleep 2
done

echo "Error: Move task ${TASK_ID} did not finish within 60s; check with: $0 task-status ${TASK_ID}" >&2
exit 1

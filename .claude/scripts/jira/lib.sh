#!/bin/bash
# Shared request function for the jira-workflow agent's scripts.
#
# Motivation: several scripts piped curl straight to jq, or printed a fixed
# {"success": true} after the call, without looking at the HTTP status. A
# rejected update was reported as made, and a failed read handed Jira's error
# body back as if it were data, both with exit code 0. Agents act on these
# results, so a failure that looks like success is the worst outcome here.

# jira_request METHOD URL [curl args...]
#
# Sends one request to Jira with the configured credentials. On a 2xx status it
# prints the response body (empty for 204) and returns 0. On any other status,
# or when Jira cannot be reached, it writes the status and Jira's error body to
# stderr and returns 1. Capture the body with BODY=$(jira_request ...) so that,
# under set -e, a failed call stops the script; piping jira_request into jq
# would hide the failure behind jq's exit code.
jira_request() {
    local method="$1"
    local url="$2"
    shift 2
    local response status body

    if ! response=$(curl -s -w '\n%{http_code}' -X "$method" \
        -u "${ATLASSIAN_USER_EMAIL}:${ATLASSIAN_API_TOKEN}" \
        "$@" \
        "$url"); then
        echo "Error: could not reach Jira for ${method} ${url}" >&2
        return 1
    fi

    status=$(printf '%s\n' "$response" | tail -n 1)
    body=$(printf '%s\n' "$response" | sed '$d')

    case "$status" in
        2??)
            printf '%s' "$body"
            return 0
            ;;
    esac

    echo "Error: Jira returned HTTP ${status} for ${method} ${url}" >&2
    if [ -n "$body" ]; then
        echo "$body" | jq '.' >&2 2>/dev/null || echo "$body" >&2
    fi
    return 1
}

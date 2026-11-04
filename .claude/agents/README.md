# Workflow agents

Two project subagents do all Jira and GitHub work for this repo. Each declares
its vendor MCP server inside its own definition, so the server connects only
while that agent runs and the main session carries none of its tools.

| Agent | Server | Kept scripts |
|---|---|---|
| `jira-workflow.md` | Atlassian Rovo MCP, `https://mcp.atlassian.com/v2/mcp` | `.claude/scripts/jira/` |
| `github-workflow.md` | GitHub MCP, `https://api.githubcopilot.com/mcp/` | `.claude/scripts/github/` |

The scripts cover the operations the servers lack: deleting issues, comments,
attachments and issue links, changing an issue's type across hierarchy levels,
uploading and downloading plain attachments, and creating a release. The rule that the main session delegates instead of calling `gh` or
the APIs itself lives in `CLAUDE.md` "Jira and GitHub".

## Setup

### 1. GitHub token

Create a personal access token with access to this repository's pull
requests, issues, Actions and contents (releases). The MCP server receives it
as a Bearer token, and `.claude/scripts/github/create-release.sh` uses it through `gh`.

### 2. Atlassian tokens

The Jira agent uses two tokens, because each works in only one place.

1. An Atlassian organization admin enables API-token sign-in for the MCP
   server: Atlassian Administration > Rovo > Rovo MCP server > Authentication.
   Until then the server answers `You don't have permission to connect via API
   token`.
2. Create a **scoped** API token for the app **Atlassian MCP V2**. This is
   `ATLASSIAN_MCP_TOKEN`. It works only against the MCP server; the REST API
   rejects it.
3. Create a **classic** API token. This is `ATLASSIAN_API_TOKEN`, used by the
   scripts in `.claude/scripts/jira/` and, as a repository secret, by the status
   workflow (step 4). The MCP server rejects it.

### 3. Environment

Put the values in the `env` block of `.claude/settings.local.json`, which git
ignores:

| Variable | Value |
|---|---|
| `GITHUB_PERSONAL_ACCESS_TOKEN` | the GitHub token |
| `ATLASSIAN_SITE_NAME` | `https://<site>.atlassian.net` |
| `ATLASSIAN_USER_EMAIL` | your Atlassian account email |
| `ATLASSIAN_MCP_TOKEN` | the scoped token |
| `ATLASSIAN_MCP_AUTH` | base64 of `email:ATLASSIAN_MCP_TOKEN`, generated below (`jq -j`, so no newline is encoded) |
| `ATLASSIAN_API_TOKEN` | the classic token |
| `JIRA_PROJECT_PREFIX` | `QUE` |
| `JIRA_PROJECT_LABEL` | `quber` |

The MCP server takes Basic auth, and an agent's header can name a variable but
cannot encode one, so `ATLASSIAN_MCP_AUTH` holds the encoded value. Generate it
from the two values already in the file, without printing it, from the repo root:

```bash
cd .claude
AUTH="$(jq -j '.env | "\(.ATLASSIAN_USER_EMAIL):\(.ATLASSIAN_MCP_TOKEN)"' settings.local.json | base64 -w0)" \
  jq '.env.ATLASSIAN_MCP_AUTH = env.AUTH' settings.local.json > settings.local.json.new \
  && mv settings.local.json.new settings.local.json
```

Run it again whenever `ATLASSIAN_MCP_TOKEN` rotates. Restart Claude Code after
any change to the file; the environment is read at session start.

### 4. Repository secrets for the status workflow

`.github/workflows/jira-transition.yml` moves an issue's Jira status on branch
and PR events. It runs on GitHub, so it reads repository secrets, not
`settings.local.json`. The secrets carry the same names and values as these
entries there: `ATLASSIAN_SITE_NAME`, `ATLASSIAN_USER_EMAIL` and the classic
`ATLASSIAN_API_TOKEN`. Set all three from the repo root:

```bash
for k in ATLASSIAN_SITE_NAME ATLASSIAN_USER_EMAIL ATLASSIAN_API_TOKEN; do
  jq -j --arg k "$k" '.env[$k]' .claude/settings.local.json | gh secret set "$k"
done
```

Rotating the classic token in `settings.local.json` does not update the
secret. Until it is updated, every run fails at its first step with
`Jira returned HTTP 401`, and statuses stop moving. Run the loop again after
each rotation.

Then re-run the failed runs, branch creation before PR opening, so the issues
they skipped catch up.

### 5. Tools and trust

- `jq`, `curl` and `gh` on the PATH, for the kept scripts.
- Accept the workspace trust prompt for the repo folder. Project agents load
  their inline MCP servers only in a trusted folder.

## Check the setup

In a new session in the repo:

1. Ask the main session to list its tools whose names start with
   `mcp__atlassian` or `mcp__github`. It has none.
2. Ask: "Have jira-workflow fetch QUE-1 and report its summary and status."
   If it reports an authentication error, check the tokens from step 2 and
   regenerate `ATLASSIAN_MCP_AUTH`.
3. Ask: "Have github-workflow list the open pull requests."

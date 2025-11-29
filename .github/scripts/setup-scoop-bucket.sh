#!/bin/bash

# Script to create and set up a Scoop bucket for standby
# Usage: ./setup-scoop-bucket.sh <github-username>

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <github-username>"
    echo "Example: $0 myusername"
    exit 1
fi

USERNAME=$1
REPO_NAME="standby"
BUCKET_REPO="${REPO_NAME}-scoop-bucket"

echo "Setting up Scoop bucket for $REPO_NAME..."
echo "GitHub username: $USERNAME"
echo "Bucket repository: $BUCKET_REPO"
echo ""

# Create bucket repository locally
echo "Creating bucket repository structure..."
mkdir -p "$BUCKET_REPO/bucket"
cd "$BUCKET_REPO"

# Initialize git repo
git init
git checkout -b main

# Create README for the bucket
cat > README.md << EOF
# ${USERNAME}/${BUCKET_REPO}

Scoop bucket for ${REPO_NAME}.

## Installation

\`\`\`bash
# Add this bucket
scoop bucket add ${USERNAME}-scoop-bucket https://github.com/${USERNAME}/${BUCKET_REPO}

# Install standby
scoop install standby
\`\`\`

## Updating

The bucket will be automatically updated when new releases are published.
EOF

# Create initial manifest (will be updated by CI)
cat > bucket/standby.json << EOF
{
    "version": "0.1.0",
    "description": "Terminal-based audio monitoring application",
    "homepage": "https://github.com/${USERNAME}/${REPO_NAME}",
    "license": "MIT",
    "architecture": {
        "64bit": {
            "url": "https://github.com/${USERNAME}/${REPO_NAME}/releases/download/v0.1.0/standby-x86_64-pc-windows-msvc.zip",
            "hash": "placeholder-sha256",
            "bin": "standby.exe"
        }
    },
    "checkver": {
        "github": "${USERNAME}/${REPO_NAME}"
    },
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://github.com/${USERNAME}/${REPO_NAME}/releases/download/v\$version/standby-x86_64-pc-windows-msvc.zip"
        }
    }
}
EOF

# Add and commit files
git add .
git commit -m "Initial commit"

echo ""
echo "Bucket repository created locally in ./${BUCKET_REPO}"
echo ""
echo "Next steps:"
echo "1. Create a new repository on GitHub named '${BUCKET_REPO}'"
echo "2. Push this local repository to GitHub:"
echo "   cd ${BUCKET_REPO}"
echo "   git remote add origin https://github.com/${USERNAME}/${BUCKET_REPO}.git"
echo "   git push -u origin main"
echo ""
echo "3. In your main repository settings, add SCOOP_BUCKET_TOKEN secret"
echo "   - Go to https://github.com/${USERNAME}/${REPO_NAME}/settings/secrets/actions"
echo "   - Add a new secret named SCOOP_BUCKET_TOKEN"
echo "   - Set it to a Personal Access Token with repo permissions"
echo ""
echo "The CI workflow will automatically update the bucket on releases!"
#!/usr/bin/env bash
# Adds the git-hook described below. Appends to the hook file
# if it already exists or creates the file if it does not.
# Note: CWD must be inside target repository

set -euxo pipefail

HOOK_DIR=$(git rev-parse --show-toplevel)/.git/hooks
HOOK_FILE="$HOOK_DIR"/pre-commit

# Create script file if doesn't exist
if [ ! -e "$HOOK_FILE" ] ; then
    echo '#!/usr/bin/env bash' >> "$HOOK_FILE"
    echo 'set -euxo pipefail' >> "$HOOK_FILE"
    chmod a+x "$HOOK_FILE"
fi
chmod a+x "$HOOK_FILE"

# Append hook code into script
cat >> "$HOOK_FILE" <<EOF
./test.sh
EOF


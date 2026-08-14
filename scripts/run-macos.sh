#!/bin/sh
# Makes Mac OS codesigning easier so that builds share a signature for
# keychain.
#
# Override MATTERMOST_CODESIGN_IDENTITY withyour identity (check
# `security find-identity -v -p codesigning` for what's available on your machine).
set -e

# If .env file exists, load it to get MATTERMOST_CODESIGN_IDENTITY
if [ -f ".env" ]; then
  source .env
fi

set +a

IDENTITY="${MATTERMOST_CODESIGN_IDENTITY}"
BINARY="$1"
shift

codesign --force --sign "$IDENTITY" "$BINARY"
exec "$BINARY" "$@"

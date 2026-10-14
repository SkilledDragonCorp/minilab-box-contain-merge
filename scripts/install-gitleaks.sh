#!/usr/bin/env bash
set -euo pipefail
VERSION="8.30.1"
ARCHIVE="gitleaks_${VERSION}_linux_x64.tar.gz"
BASE="https://github.com/gitleaks/gitleaks/releases/download/v${VERSION}"
EXPECTED_SHA256="551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
curl -fsSL "${BASE}/${ARCHIVE}" -o "${TMP}/${ARCHIVE}"
curl -fsSL "${BASE}/gitleaks_${VERSION}_checksums.txt" -o "${TMP}/checksums.txt"
GOT="$(sha256sum "${TMP}/${ARCHIVE}" | awk '{print $1}')"
if [[ "$GOT" != "$EXPECTED_SHA256" ]]; then
  echo "gitleaks checksum mismatch: got $GOT want $EXPECTED_SHA256"
  exit 1
fi
grep -F "$EXPECTED_SHA256  ${ARCHIVE}" "${TMP}/checksums.txt" >/dev/null
tar -xzf "${TMP}/${ARCHIVE}" -C "$TMP" gitleaks
install -m 0755 "${TMP}/gitleaks" /usr/local/bin/gitleaks
gitleaks version

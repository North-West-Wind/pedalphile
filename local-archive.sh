#!/usr/bin/env sh

tar --exclude="pkgbuild" --exclude="local-archive.sh" --exclude="target" -czvf pkgbuild/pedalphile.tar.gz ./*
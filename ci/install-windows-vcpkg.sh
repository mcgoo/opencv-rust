#!/bin/bash

# WIP, doesn't work yet

set -vex

choco install -y llvm --version 9.0.0

export VCPKG_ROOT="$HOME/build/vcpkg"
if [ ! -e "$VCPKG_ROOT" ]; then
	git clone --depth=1 https://github.com/Microsoft/vcpkg.git "$VCPKG_ROOT"
fi
pushd "$VCPKG_ROOT"
git pull
cmd "/C bootstrap-vcpkg.bat -disableMetrics"
#./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x86-windows.cmake
export VCPKG_DEFAULT_TRIPLET=x64-windows
# this is going to output something to console while opencv builds
# otherwise travis terminates job because of terminal inactivity
#( while true; do sleep 5m; done ) &
./vcpkg install "opencv${VCPKG_OPENCV_VERSION}[contrib,nonfree]"
./vcpkg upgrade
#kill $(jobs -p)
popd


Rusty Battleships
=================

Client/Server implementation of Battleship (Rust, group 6)


Build requirements
------------------

 * Rust 1.6 (on Windows: 64-bit MSVC ABI)
 * CMake
 * C++ compiler and build tools:
   * Linux: g++ (4.8+) and make
   * OS X: XCode command line tools (include clang)
   * Windows: Visual Studio 2015, including C++ support
 * Qt 5.5 (Linux, OS X) / 5.6 beta, 64-bit, VS 2015 (Windows)
   * When using the online installer only the following Qt modules have to be selected: 
     - Quick Controls
     - Quick
     - Script

Environment setup on Windows
----------------------------

 * Set the QTDIR environment variable to `{$QT_INSTALL_DIR}\5.6\msvc2015_64`
 * Add `{$QT_INSTALL_DIR}\5.6\msvc2015_64\bin` to the path (contains .dlls)

The above steps should become unnecessary once Qt 5.6 stable is released,
since the installer for stable versions should set up the environment
automatically. However, Qt 5.5 does not support Visual Studio 2015, so we have
to use the beta to build on Windows at the moment.

Environment setup on OS X
----------------------------

Qt (QtQuick and base libraries) must be installed via the official installer,
the Homebrew version does not work. The following environment variables need to
be set:

```bash
CMAKE_PREFIX_PATH=$QTDIR
PKG_CONFIG_PATH=$QTDIR/lib/pkgconfig
DYLD_FRAMEWORK_PATH=$QTDIR/lib
```

Ubuntu packages
---------------

The following Ubuntu packages must be installed to compile the project:

* qml-module-qtquick-controls
* qml-module-qtquick-dialogs
* qtbase5-dev
* qtdeclarative5-dev

They are available in Ubuntu 15.04 and newer versions. For older versions, such as 14.04 LTS, use the online installer. In this case, export the following, after replacing <QT-Path> and <QT-Version> qith the path to your Qt installation and your Qt version:

$ export CMAKE_PREFIX_PATH=<QT-Path>/<QT-Version>/gcc_64/
$ export QTDIR=<QT-Path>/<QT-Version>/gcc_64/
$ export DYLD_FRAMEWORK_PATH=<QT-Path>/<QT-Version>/gcc_64/lib
$ export PKG_CONFIG_PATH=<QT-Path>/<QT-Version>/gcc_64/lib/pkgconfig/

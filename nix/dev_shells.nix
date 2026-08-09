# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

_:

{
    perSystem =
        {
            libPath,
            nativeBuildInputs,
            pkgs,
            runtimeDependencies,
            toolchain,
            ...
        }:
        {
            devShells.default = pkgs.mkShell {
                inherit nativeBuildInputs;
                LD_LIBRARY_PATH = libPath;
                buildInputs = runtimeDependencies;
                packages = [ toolchain ];
            };
        };
}

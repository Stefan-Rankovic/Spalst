# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    availableProfiles,
    defaultFeatures,
    defaultProfile,
    inputs,
    ...
}:

{
    perSystem =
        {
            inputs',
            lib,
            pkgs,
            ...
        }:
        let
            commonCraneArgs = {
                inherit nativeBuildInputs;
                cargoExtraArgs = "-Zcargo-lints";
                pname = "spalst";
                src = craneLib.cleanCargoSource ../.;
                strictDeps = true;
            };
            craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
            featuresExtraArgs =
                features:
                assert builtins.isList features;
                "--no-default-features${
                    lib.optionalString (
                        (builtins.length features) > 0
                    ) " --features ${lib.concatStringsSep "," features}"
                }";
            nativeBuildInputs = with pkgs; [
                clang # Needed for `mold`
                mold
            ];
            runtimeDependencies = with pkgs; [
                wayland
                libxkbcommon
                libGL
            ];
            toolchain = inputs'.fenix.packages.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
            ];
        in
        {
            _module.args = {
                inherit
                    commonCraneArgs
                    craneLib
                    featuresExtraArgs
                    nativeBuildInputs
                    runtimeDependencies
                    toolchain
                    ;
                libPath = lib.makeLibraryPath runtimeDependencies;
                cargoArtifacts =
                    {
                        features ? defaultFeatures,
                        profile ? defaultProfile,
                    }:
                    assert builtins.isString profile;
                    assert builtins.elem profile availableProfiles;
                    assert builtins.isList features;
                    craneLib.buildDepsOnly (
                        commonCraneArgs
                        // {
                            CARGO_PROFILE = profile;
                            RUSTFLAGS = "-Awarnings"; # Necessary as otherwise the dependencies aren't cached
                            cargoExtraArgs = featuresExtraArgs features;
                        }
                    );
            };
        };
}

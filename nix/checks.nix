# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    allFeatures,
    ...
}:

{
    perSystem =
        {
            cargoArtifacts,
            commonCraneArgs,
            craneLib,
            ...
        }:
        let
            mkClippyCheck =
                { name, extraArgs }:
                assert builtins.isString name;
                assert builtins.isList extraArgs;
                craneLib.cargoClippy (
                    commonCraneArgs
                    // {
                        cargoArtifacts = cargoArtifacts {
                            features = allFeatures;
                            profile = "dev";
                        };
                        cargoClippyExtraArgs = builtins.concatStringsSep " " (
                            [
                                "--all-targets"
                                "--all-features"
                                "-Zcargo-lints"
                            ]
                            ++ extraArgs
                        );
                        pname = "spalst-clippy-${name}";
                    }
                );
        in
        {
            checks = {
                clippy-strict = mkClippyCheck {
                    name = "strict";
                    extraArgs = [ ]; # The `Cargo.toml` lint configuration is already a bit strict by default
                };

                clippy-relaxed = mkClippyCheck {
                    name = "relaxed";
                    extraArgs = [
                        "--"
                        "--allow clippy::multiple_crate_versions"
                        "--allow unused"
                    ];
                };
            };
        };
}

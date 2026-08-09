# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    availableProfiles,
    defaultFeatures,
    ...
}:

{
    perSystem =
        {
            cargoArtifacts,
            commonCraneArgs,
            craneLib,
            featuresExtraArgs,
            lib,
            libPath,
            nativeBuildInputs,
            pkgs,
            ...
        }:
        {
            _module.args.mkCranePackage =
                {
                    features ? defaultFeatures,
                    profile,
                }:
                assert builtins.isList features;
                assert builtins.isString profile;
                assert builtins.elem profile availableProfiles;
                craneLib.buildPackage (
                    commonCraneArgs
                    // {
                        CARGO_PROFILE = profile;
                        RUSTFLAGS = "-Awarnings"; # Prevent unnecessary build output; this can be easily checked with `cargo clippy` or `cargo check`
                        cargoArtifacts = cargoArtifacts { inherit features profile; };
                        cargoExtraArgs = "-Zcargo-lints ${featuresExtraArgs features}";
                        nativeBuildInputs = nativeBuildInputs ++ [ pkgs.makeWrapper ];
                        meta =
                            let
                                githubRepo = "https://github.com/Stefan-Rankovic/Spalst";
                            in
                            {
                                # description = ""; # todo
                                license = lib.licenses.gpl3Plus;
                                homepage = githubRepo;
                                downloadPage = "${githubRepo}/releases";
                                # changelog = ""; # todo
                                sourceProvenance = [ lib.sourceTypes.fromSource ];
                                mainProgram = "spalst";
                            };
                        postInstall =
                            let
                                binaryPath = "$out/bin/spalst";
                                actualBinaryPath = "$out/binary";
                            in
                            ''
                                mv ${binaryPath} ${actualBinaryPath}
                                makeWrapper "${actualBinaryPath}" "${binaryPath}" --prefix LD_LIBRARY_PATH : "${libPath}"
                            '';
                    }
                );
        };
}

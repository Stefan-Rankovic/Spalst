# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

        crane.url = "github:ipetkov/crane";
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs =
        {
            crane,
            fenix,
            nixpkgs,
            ...
        }:
        let
            profiles = [
                "dev"
                "release"
            ];
            defaultProfile = "release";

            runtimeDependencies = with pkgs; [
                wayland
                libxkbcommon
                libGL
            ];
            inherit (nixpkgs) lib;
            pkgs = import nixpkgs {
                system = "x86_64-linux";
                overlays = [ fenix.overlays.default ];
            };
            toolchain = pkgs.fenix.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
            ];
            craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
            src = craneLib.cleanCargoSource ./.;
            libPath = lib.makeLibraryPath runtimeDependencies;
            pname = "spalst";
            isValidProfile =
                profile:
                assert builtins.isString profile;
                builtins.elem profile profiles;
            mkCargoArtifacts =
                profile:
                assert isValidProfile profile;
                craneLib.buildDepsOnly {
                    inherit pname src;
                    CARGO_PROFILE = profile;
                };
            mkCranePackage =
                profile:
                assert isValidProfile profile;
                craneLib.buildPackage {
                    inherit pname src;
                    nativeBuildInputs = with pkgs; [
                        makeWrapper
                    ];
                    postInstall = ''
                        wrapProgram "$out/bin/spalst" --prefix LD_LIBRARY_PATH : "${libPath}"
                    '';
                    cargoExtraArgs = "-Zcargo-lints";
                    RUSTFLAGS = "-Awarnings"; # Prevent unnecessary build output; this can be easily checked with `cargo clippy` or `cargo check`
                    cargoArtifacts = mkCargoArtifacts profile;
                    CARGO_PROFILE = profile;
                };

        in
        {
            packages.${pkgs.stdenv.hostPlatform.system} =
                lib.attrsets.unionOfDisjoint
                    {
                        default = mkCranePackage defaultProfile;
                    }
                    (
                        builtins.listToAttrs (
                            map (
                                profile:
                                assert builtins.isString profile;
                                {
                                    name = profile;
                                    value = mkCranePackage profile;
                                }
                            ) profiles
                        )
                    );
            devShells.${pkgs.stdenv.hostPlatform.system}.default = pkgs.mkShell {
                packages = with pkgs; [
                    toolchain
                    gcc
                ];
            };
        };
}

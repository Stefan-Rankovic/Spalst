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
            system = "x86_64-linux";
            pkgs = import nixpkgs {
                inherit system;
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
            libPath = nixpkgs.lib.makeLibraryPath (
                with pkgs;
                [
                    wayland
                    libxkbcommon
                    libGL
                ]
            );
        in
        {
            packages.${system}.default = craneLib.buildPackage {
                pname = "spalst";
                src = craneLib.cleanCargoSource ./.;
                nativeBuildInputs = with pkgs; [
                    makeWrapper
                ];
                postInstall = ''
                    wrapProgram "$out/bin/spalst" --prefix LD_LIBRARY_PATH : "${libPath}"
                '';
                cargoExtraArgs = "-Zcargo-lints";
                RUSTFLAGS = "-Awarnings";
            };
            devShells.${system}.default = pkgs.mkShell {
                packages = with pkgs; [
                    toolchain
                    gcc
                ];
            };
        };
}

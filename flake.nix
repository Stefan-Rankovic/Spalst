# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    # description = ""; # todo

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-parts.url = "github:hercules-ci/flake-parts";

        crane.url = "github:ipetkov/crane";
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs =
        inputs@{
            flake-parts,
            ...
        }:
        flake-parts.lib.mkFlake { inherit inputs; } {
            imports = [
                ./nix/modules

                ./nix/checks.nix
                ./nix/compile_options_data.nix
                ./nix/dev_shells.nix
                ./nix/legacy_packages.nix
                ./nix/mk_crane_package.nix
                ./nix/packages.nix
                ./nix/per_system_module_args.nix
            ];

            systems = [ "x86_64-linux" ];
        };
}

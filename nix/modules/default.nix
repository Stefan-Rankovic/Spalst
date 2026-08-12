# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    allFeatures,
    availableProfiles,
    defaultFeatures,
    defaultProfile,
    self,
    ...
}:

{
    flake =
        _:
        let
            mkModule =
                installPackage:
                {
                    config,
                    lib,
                    pkgs,
                    ...
                }:

                let
                    cfg = config.programs.spalst;
                in
                {
                    options.programs.spalst = import ./options.nix {
                        inherit
                            allFeatures
                            availableProfiles
                            defaultFeatures
                            defaultProfile
                            lib
                            ;
                    };

                    config = lib.mkIf cfg.enable (
                        installPackage (
                            self.legacyPackages.${pkgs.stdenv.hostPlatform.system}."${cfg.compilationOpts.profile}WithFeatures"
                                cfg.compilationOpts.features
                        )
                    );
                };
        in
        {
            nixosModules.default = mkModule (pkg: {
                environment.systemPackages = [ pkg ];
            });

            homeManagerModules.default = mkModule (pkg: {
                home.packages = [ pkg ];
            });
        };
}

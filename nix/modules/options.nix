# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    availableFeatures,
    availableProfiles,
    defaultFeatures,
    defaultProfile,
    lib,
}:

let
    inherit (lib) mkEnableOption mkOption types;
in
{
    enable = mkEnableOption "spalst";

    compilationOpts = mkOption {
        type = types.submodule {
            options = {
                profile = mkOption {
                    type = types.enum availableProfiles;
                    default = defaultProfile;
                };

                features = mkOption {
                    type = types.listOf (types.enum availableFeatures);
                    default = defaultFeatures;
                };
            };
        };
        default = { };
        description = "Compilation options for the spalst package";
    };
}

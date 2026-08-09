# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    allFeatures,
    availableProfiles,
    defaultProfile,
    ...
}:

{
    perSystem =
        {
            lib,
            mkCranePackage,
            ...
        }:
        {
            packages =
                lib.attrsets.unionOfDisjoint
                    {
                        default = mkCranePackage { profile = defaultProfile; };
                        noFeatures = mkCranePackage {
                            features = [ ];
                            profile = defaultProfile;
                        };
                        allFeatures = mkCranePackage {
                            features = allFeatures;
                            profile = defaultProfile;
                        };
                    }
                    (
                        builtins.listToAttrs (
                            builtins.concatLists (
                                map (
                                    profile:
                                    assert builtins.isString profile;
                                    [
                                        {
                                            name = profile;
                                            value = mkCranePackage { inherit profile; };
                                        }
                                        {
                                            name = "${profile}NoFeatures";
                                            value = mkCranePackage {
                                                inherit profile;
                                                features = [ ];
                                            };
                                        }
                                        {
                                            name = "${profile}AllFeatures";
                                            value = mkCranePackage {
                                                inherit profile;
                                                features = allFeatures;
                                            };
                                        }
                                    ]
                                ) availableProfiles
                            )
                        )
                    );
        };
}

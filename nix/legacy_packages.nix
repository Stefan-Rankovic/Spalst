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
            mkCranePackage,
            ...
        }:
        {
            legacyPackages = builtins.listToAttrs (
                builtins.concatLists (
                    map (
                        profile:
                        assert builtins.isString profile;
                        [
                            {
                                name = "${profile}WithFeatures";
                                value =
                                    features:
                                    assert builtins.isList features;
                                    mkCranePackage { inherit features profile; };
                            }
                            {
                                name = "${profile}WithAdditionalFeatures";
                                value =
                                    additionalFeatures:
                                    assert builtins.isList additionalFeatures;
                                    mkCranePackage {
                                        inherit profile;
                                        features = defaultFeatures ++ additionalFeatures;
                                    };
                            }
                        ]
                    ) availableProfiles
                )
            );
        };
}

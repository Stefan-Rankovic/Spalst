#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-only

clear
cargo run -Zcargo-lints -- --clean-previous-logs --log-level trace
bat logs/*


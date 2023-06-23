#!/bin/bash
# Copyright Kani Contributors
# SPDX-License-Identifier: Apache-2.0 OR MIT

set -eux

cargo install mdbook-graphviz
apt-get install graphviz

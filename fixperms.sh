#!/bin/bash
sudo chown root:root target/debug/elevate && sudo chmod u+s target/debug/elevate
sudo chown root:root target/release/elevate && sudo chmod u+s target/release/elevate


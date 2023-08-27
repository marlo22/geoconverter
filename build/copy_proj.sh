#!/bin/bash

destination_path="../"

cd target/release/build

for dir in proj-sys*; do
    if [ -d "$dir/out" ]; then
      cp -r "$dir/out/share/proj" "$destination_path"
    fi
done

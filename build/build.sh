cargo build --release &&
./build/copy_proj.sh &&
cd target/release &&
zip -r geoconverter.zip proj geoconverter &&
echo "Success! Build path: target/release/geoconverter.zip"

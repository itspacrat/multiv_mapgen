echo "-------------------------------------"
echo "* updating the database file via git..."
if [ -d "../db_gist" ]
then
    cd ../db_gist && git pull
else
    git clone https://gist.github.com/itspacrat/c4557e52b681bc0ac35b361f08a61597.git ../db_gist
fi
cd ../multiv_mapgen
echo "* copying to db.json..."
cp -v ../db_gist/multiv_db.json db.json;
echo "updated database file. processing maps:"
cat "process.json" && echo ""
export CARGO_CONFIG_HOME="."
cargo run --release
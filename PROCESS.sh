echo "-------------------------------------"
echo "* updating the database file via git..."
if [ -d "../db_gist" ]
then
    cd ../db_gist && git pull
else
    git clone https://gist.github.com/3aea75ef1f97e25f9c9284e6914e993e.git ../db_gist
fi
cd ../multiv_mapgen
echo "* copying to db.json..."
cp -v ../db_gist/lvsten_db_shortrgb.json db.json;
echo "updated database file. processing maps:"
cat "process.json" && echo ""
cargo run
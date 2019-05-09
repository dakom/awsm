#!/bin/sh

# prepare all-inclusive site folder
rm -rf ./dist
mkdir dist 
cp -R ./site/* ./dist/

# bundle and copy integration-tests
cd examples/integration-tests
npm run bundle
cp -R dist ../../dist/integration-tests
cd ../..

# manually deploy all-inclusive site folder
netlify deploy --prod --dir dist

echo ""
echo "DON'T FORGET TO PUBLISH VIA THE NETLIFY SITE!!"
echo ""


# This scripts remove all files produced by the main program & the stats tool
#
# Useful when permission issues occur over fodlers created by 
# gathering / processing scripts

PROJECTDIR=$(dirname "$0")/..

rm -rf $PROJECTDIR/out
rm -rf $PROJECTDIR/tmp
rm -rf $PROJECTDIR/*.png
rm -rf $PROJECTDIR/*.dat
rm -rf $PROJECTDIR/*.data
rm -rf $PROJECTDIR/*.old
rm -rf $PROJECTDIR/*.csv
rm -rf $PROJECTDIR/*.svg
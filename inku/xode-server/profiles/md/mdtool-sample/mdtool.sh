#!/bin/bash

cp_dir(){
    local FROM=$1
    local TO=$2
    rm -rf $TO
    mkdir -p $TO
    cp -R $FROM/. $TO
}

BUILD_LOG=mdtool.log

plantuml(){
    STDIN=$(cat)
    echo "=== Build plantuml" > $BUILD_LOG
    date >> $BUILD_LOG
    env >> $BUILD_LOG
    echo $STDIN | jq . >> $BUILD_LOG
    # echo $STDIN | jq .[1].sections[].Chapter.content >> $BUILD_LOG
    # echo $STDIN | jq .[1].sections[].Chapter.content | grep -o '```plantuml\\n@startuml\\n.*enduml\\n```' > key.txt
    # cat key.txt | sed 's/```plantuml\\n//g ; s/```//g' > target.txt
    # Only return STDIN JSON .[1] object
    echo $STDIN | jq .[1] | sed 's/Alice/Foo/g'
}

usage() {
    cat 1>&2 <<EOF
The script for shTemplate
USAGE:
    bash shTemplate.sh [FLAGS] [OPTIONS]
FLAGS:
    -v, --version           Prints version information
EOF
}

case "$1" in
  plantuml) shift; plantuml $@ ;;
  -h|--help)
      usage
      exit 0
      ;;
  *) usage
     exit 0
     ;;
esac
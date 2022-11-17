#/usr/bin/env sh
INPUT_DIR=examples
OUTPUT_DIR=$INPUT_DIR/outputs
EXECUTABLE=target/debug/st

cargo build

if [ ! -d $OUTPUT_DIR ] ; then
    mkdir $OUTPUT_DIR
fi

for FILE in examples/* ; do
    if [ -f $FILE ] ; then
        OUTPUT_PATH=$OUTPUT_DIR/$(echo $FILE | cut -d "/" -f2 | cut -d "." -f1).txt
        printf "Running file $FILE"
        $EXECUTABLE $FILE > $OUTPUT_PATH
        printf ". Done output written to $OUTPUT_PATH\n"
    fi
done
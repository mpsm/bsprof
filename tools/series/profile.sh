#!/usr/bin/sh

TITLE=$1
CMD=$2
CLEAN=$3

time_util=$(which time)
report="profile.$(date +%s).txt"

echo "${TITILE}" > $report

for i in $(seq $(nproc)); do
	echo ">> Building with $i threads"
	cmd="$(echo $CMD | sed 's,NPROC,'$i',')"
	echo ">> $cmd"
	$CLEAN
	echo "#$i" >> $report
	$time_util -f "@%e,%S,%U,%P,%C" --output=$report --append $cmd
done


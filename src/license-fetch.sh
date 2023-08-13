#!/bin/sh

cd $(dirname $0)

LICENSE_KEYWORDS=$(sed -n '/pub const LICENSES/,/];/p' licenses.rs | grep -oP '(?<=, \").*(?=\"\))')

CURRENT=1
MAX=$(echo ${LICENSE_KEYWORDS} | wc -w)

QUEUE_SIZE=10
TMP_DIR=$(mktemp -d)

RED='\033[1;31m'
BLUE='\033[1;34m'
GREEN='\033[0;32m'
NC='\033[0m'

download() {
    curl -s "https://raw.githubusercontent.com/spdx/license-list-data/main/text/$1.txt" -o license-templates/$1.temp

    if [[ $(cat license-templates/$1.temp) == "404: Not Found"* ]]
    then
        echo -e "License ${BLUE}$1${NC} : ${RED}404 Not Found${NC}" > ${TMP_DIR}/RESULT$2
        rm license-templates/$1.temp
    else
        echo -e "License ${BLUE}$1${NC} : ${GREEN}Downloaded${NC}" > ${TMP_DIR}/RESULT$2
        mv license-templates/$1.temp license-templates/$1
    fi
}

cleanup() {
    rm -rf ${TMP_DIR}
    kill $(jobs -p) > /dev/null 2>&1
    exit
}

trap cleanup SIGINT SIGTERM EXIT

echo "Staring download queue with queue size ${QUEUE_SIZE}"

for word in ${LICENSE_KEYWORDS}
do
    while [[ $(jobs | wc -l ) -gt $((${QUEUE_SIZE} - 1)) ]]
    do
        sleep 0.1
    done

    echo -e "Downloading ${BLUE}${word}${NC} (${CURRENT}/${MAX})"
    download ${word} ${CURRENT} &
    CURRENT=$((${CURRENT} + 1))
done

echo -e "Waiting for dispatched jobs to complete\n\n"
wait

for ((i=1; i < ${MAX}; i++))
do
    cat ${TMP_DIR}/RESULT${i}
done

cleanup
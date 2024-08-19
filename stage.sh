#!/bin/bash
# Simple bash script to stage results from chatdownloader to the
# webpage

CHATDOWNLOADER="rust"
WEB="web/static"

cp ${CHATDOWNLOADER}/*.bin ${WEB}/

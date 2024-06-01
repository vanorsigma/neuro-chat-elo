#!/bin/bash
# Simple bash script to stage results from chatdownloader to the
# webpage

CHATDOWNLOADER="chatdownloader"
WEB="web/static"

cp ${CHATDOWNLOADER}/*.json ${WEB}/

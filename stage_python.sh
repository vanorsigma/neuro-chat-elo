#!/bin/bash
# Simple bash script to stage results from chatdownloader to the
# webpage

CHATDOWNLOADER="python-chatdownloader"
WEB="web/static"

cp ${CHATDOWNLOADER}/*.json ${WEB}/

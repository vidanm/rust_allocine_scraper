#!/bin/sh
echo "Installing geckodriver and then running it"
# Get all the prereqs

cd /homes
mkdir .mozilla
cd .mozilla
mkdir firefox
geckodriver -b /user/bin/firefox &&
# apk add xterm
# exec /usr/bin/firefox -no-remote
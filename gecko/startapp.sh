#!/bin/sh
firefox --headless --marionette -start-debugger-server 2828 &\
geckodriver -b /usr/bin/firefox --connect-existing --marionette-port 2828 --host 0.0.0.0 --allow-hosts "geckodriver"

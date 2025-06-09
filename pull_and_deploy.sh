#!/bin/bash

rm -f cyberpunk-app.tar.gz
sudo wget https://github.com/benjo121ben/cyberpunk_red_char_sheet/releases/latest/download/cyberpunk-app.tar.gz &&
tar -xf cyberpunk-app.tar.gz &&
sudo systemctl restart cyberpunk.service

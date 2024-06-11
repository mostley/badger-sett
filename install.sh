#!/bin/sh
set -x

REMOTE_USER=root
REMOTE_HOST=badger

# Create data dir
ssh $REMOTE_USER@$REMOTE_HOST "mkdir -p /var/lib/badger-sett"

# Stop the service so we can update the binary
ssh $REMOTE_USER@$REMOTE_HOST systemctl stop badger-sett

# Install the binary
scp target/x86_64-unknown-linux-gnu/release/badger-sett $REMOTE_USER@$REMOTE_HOST:/usr/local/bin/badger-sett

# Install the config file
scp install/Rocket.toml $REMOTE_USER@$REMOTE_HOST:/var/lib/badger-sett/

# Install the service file
scp install/badger-sett.service $REMOTE_USER@$REMOTE_HOST:/etc/systemd/system/

# Reload and restart the service
ssh $REMOTE_USER@$REMOTE_HOST systemctl daemon-reload
ssh $REMOTE_USER@$REMOTE_HOST systemctl enable --now badger-sett

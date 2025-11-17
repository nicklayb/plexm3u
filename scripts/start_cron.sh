#!bin/sh

echo "$CRON sync_all.sh" > /crontab
supercronic /crontab

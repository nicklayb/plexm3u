#!/bin/sh

TOKEN_ARGS=""
[ -n "$PLEX_TOKEN" ] && TOKEN_ARGS="--token $PLEX_TOKEN"

PLEX_SERVER_ARGS="--server $PLEX_SERVER $TOKEN_ARGS"

if [ -z "$PLAYLISTS" ]; then
    echo "No PLAYLISTS provided, fetching all playlists..."
    while read -r line; do
        if echo "$line" | grep -qE '^[0-9]+'; then
            id=$(echo "$line" | awk -F':' '{print $1}')
            PLAYLISTS="$PLAYLISTS $id"
        fi
    done < <(plexm3u list-playlists $PLEX_SERVER_ARGS)
fi

for playlist in $PLAYLISTS; do
    echo "Dumping playlists $playlist..."
    plexm3u dump-playlist $PLEX_SERVER_ARGS -f $DESTINATION $playlist
done


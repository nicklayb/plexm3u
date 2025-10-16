# Plexm3u

Client that generates M3U playlist from a Plex server.

### My (quite niche) use case for this

I've always loved to have my music on an actual device that doesn't depend on the network for a lot of reasons. Yes, Plexamp is amazing, yes, I have CarPlay that allows me to listen to Plexamp. But the experience, is not comparable to how the native audio system integrates with the overall car. I hate touchscreens so if I can avoid using CarPlay to browse songs, the better.

My use case is that my car (a 2019 Audi e-tron) has an amazing audio system. The MMI is really nice to use, you can browse tracks with the steering wheel and even add shortcuts in the climate control to start playlists or artists (I obviously have a shortcut that starts Rush songs on shuffle).

So this tool allows me to easily sync my playlists from Plexamp straight to my USB drive that has all my music.

**Note**: This was only tested with my car's stereo. So maybe there can be issues with your particular use case

## Usage

The tool supports various command:

### List playlists

```
plexm3u list-playlists --server $PLEX_SERVER --token $PLEX_TOKEN
```

The `--token` argument can be ommitted if you're pointing directly to your Plex server. You can also pass `--only audio` to list only audio playlists.

The list will look like:

```
120110: All Music [audio] [Smart] [15376 tracks]
135542: Best Albums [audio] [Smart] [1589 tracks]
135420: HipHop Rap [audio] [Smart] [2028 tracks]
135422: Québec [audio] [Smart] [3403 tracks]
120109: Recently Added [audio] [Smart] [84 tracks]
120107: Recently Played [audio] [Smart] [452 tracks]
130537: Adrien’s Early Days [audio] [851 tracks]
...
```

The number on the left is the "Rating Key", it'll be necessary to get or dump a playlist.

### Dump a playlist

```
plexm3u dump-playlist --server $SERVER -f [Destination M3U file] [Rating key]
```

#### Rewriting path

Two other arguments you can find useful are `--rewrite-from` and `--rewrite-to`. These allow to rewrite the path to match you destination's directory structure.

Suppose your server runs in a Docker container (like mine), your folder structure will likely need to be rewritten. Also given that M3U playlists have relative path to the playlist.

##### Concrete example

My server has `/music/iTunes/Rush/Power Windows/03 Manhattan Project.m4a`. But on my thumb drive, all artists are in a Music folder, the same (wonderful) song would be in `Music/Rush/Power Windows/03 Manhattan Project.m4a`.

In order to dump a playlist, I need to do 

```
plexm3u dump-playlist --server $SERVER -f /media/usb/playlist.m3u --rewrite-from "/music/iTunes" --rewrite-to "Music` [Rating key]
```

### Verifying an M3u file

Once the playlist is dumped you can verify that your drive indeed has the files. This can help showing disparity between your drive and what's on your server.

A concrete example here, when I ran the verifying for a playlist, I got the following

```
plexm3u verify-m3u -f /media/usb/patate.m3u

The following tracks (1 out of 70) could not be found at "/media/usb"
	Music/Peter Henry Phillips/Peter Henry Phillips - EP/03 Secret.m4a
```

This is because on my drive, the album is named "Peter Henry Phillips" (Missing the EP).

By default, the command searches in the m3u's folder. But if there's a case where the m3u is elsewhere, you can pass `-p [folder where tracks are located]`.

## TODO

- [] Ability to copy (or sync) files.
- [] Improve ugly `get-playlist` output

#!/bin/bash

rm -fr ./game/assets
rm -fr ./game/client/assets
rm -fr ./game/server/assets

ln -sr ./assets ./game/assets
ln -sr ./assets ./game/client/assets
ln -sr ./assets ./game/server/assets


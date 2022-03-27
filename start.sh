#!/bin/bash

docker run -it --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/coding --name rustcoding  bash

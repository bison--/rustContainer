@echo off
REM this should work but don't, so whatever
REM --user "NT AUTHORITY\SYSTEM"

docker run --rm -e USER=user -it -v %cd%:/coding rustcoding bash

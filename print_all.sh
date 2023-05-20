#!/bin/bash

# A small script to run pfetch (command specified with arguments) with all available logos

while read -r logo; do
    PF_ASCII=$logo "$@"
done << EOF
alma
alpine
android
amogos
arch
arco
artix
bedrock
buildroot
celos
centos
crystallinux
dahlia
debian
devuan
dietpi
dragonfly
elementary
endeavour
fedora
fiwix
freebsd
garuda
gentoo
gnu
guix
haiku
hydroOS
hyperbola
iglunix
instantos
irix
kdeneon
linuxlite
linuxmint
linux
macos
mageia
manjaro
morphos
minix
mx
netbsd
nixos
nobara
openbsd
opensusetumbleweed
opensuse
openwrt
oracle
parabola
pop!_os
pureos
raspbian
rocky
serenityos
slackware
solus
steamos
solaris
ubuntu
vanilla
void
windows
xeonix
EOF


We are always happy to see and hear that people want to contribute to this project. Bug fixes, new distros, or new designs, are always welcome.

# Prerequisites
Before you add anything check first if there is an issue regarding your addition, feature, bug fix, or others. If there is none please create one before you add your pull request. If your issue is regarding a bug, it would be helpful to first check one of the main dependencies of the project and check their issues there.  

Please check your changes locally before you create a pull request.

# Updating a distro icon
we understand our ASCII art may not always be the best. If you want to update our ASCII art you can do it in the [logos.sh](pfetch-extractor/logos.sh) file.

We try to keep it in 8 lines of lower, given that the visual identity of the logo remains. After that you will need to add your contribution to the README using the format "distro_name *(updated)*"

# Adding a distribution
Adding a distribution is almost as easy as updating the icon. You need to do the following:
- Add your distribution in [print_all.sh](print_all.sh), note it has to be in alphabetical order.
- Add your icon in [logos.sh](pfetch-extractor/logos.sh), try to keep it in 8 lines or less.
- Add your icon to the README using the following format, "disto_name *(new)*"

# Bugs and bug fixes
This repository's main dependency is [libmacchima](https://github.com/Macchina-CLI/libmacchina), many of the issues we have ran into have been related to this. If you find that your issue is related to libmacchina please don't hesitate to add an issue there as well as add a PR if possible.

However if the issue is in fact related to us, feel free to add your issue and PR. Please make sure it works locally first.

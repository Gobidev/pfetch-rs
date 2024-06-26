#!/bin/bash

# Most of the below logos are from pfetch by Dylan Araps licensed under the MIT License
# The MIT License (MIT)

# Copyright (c) 2016-2019 Dylan Araps
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

case ${1:-${PF_ASCII:-${distro:-$os}}} in
	[Aa]lma*)
		read_ascii 3 <<- EOF
				${c1}    {#@ .,  ${c3} 	,..<.
				${c1}    ._\`=#  ${c3}/#=#,""
				${c1}   \\##  \` ${c3}|\`  '=##
				${c4}   .${c1}\`'=.  ${c3}\\  ${c2},_,.,
				${c4} .."#-   ${c1}\`  ${c2}\`    \\##
				${c4}\## #   ./  ${c6}\\.  ${c2}.#\`,+
				${c4}   =##=' ${c6},   |, ${c2}|# ""
				${c6}       =##=++#|
				${c6}         ##\\''
				${c6}         \`'\`
			EOF
		;;

    [Aa]lpine*)
        read_ascii 4 <<- EOF
				${c4}   /\\ /\\
				  /${c7}/ ${c4}\\  \\
				 /${c7}/   ${c4}\\  \\
				/${c7}//    ${c4}\\  \\
				${c7}//      ${c4}\\  \\
				         ${c4}\\
			EOF
        ;;

    [Aa]ndroid*)
        read_ascii 2 <<- EOF
				${c2}  ;,           ,;
				${c2}   ';,.-----.,;'
				${c2}  ,'           ',
				${c2} /    O     O    \\
				${c2}|                 |
				${c2}'-----------------'
			EOF
        ;;

    [Aa]rch*)
        read_ascii 4 <<- EOF
				${c6}       /\\
				${c6}      /  \\
				${c6}     /\\   \\
				${c4}    /      \\
				${c4}   /   ,,   \\
				${c4}  /   |  |  -\\
				${c4} /_-''    ''-_\\
			EOF
        ;;

    [Aa]mog[Oo][Ss]*)
        read_ascii 4 <<- EOF
				${c7}      -///:.
				${c7}     smhhhhmh\`
				${c7}    :NA${c4}mogO${c7}SNs
				${c7}    hNNmmmmNNN
				${c7}    NNNNNNNNNN
				${c7}   :NNNNNNNNNN
				${c7}   mNNssussNNN
				${c7}  sNn:    sNNo
				${c7}+ooo+     sNNo
				${c7}        +oooo\`
			EOF
        ;;

    [Aa]rco*)
        read_ascii 4 <<- EOF
				${c4}      /\\
				${c4}     /  \\
				${c4}    / /\\ \\
				${c4}   / /  \\ \\
				${c4}  / /    \\ \\
				${c4} / / _____\\ \\
				${c4}/_/  \`----.\\_\\
			EOF
        ;;

    [Aa]rtix*)
        read_ascii 6 <<- EOF
				${c4}      /\\
				${c4}     /  \\
				${c4}    /\`'.,\\
				${c4}   /     ',
				${c4}  /      ,\`\\
				${c4} /   ,.'\`.  \\
				${c4}/.,'\`     \`'.\\
			EOF
        ;;

    [Bb]azzite*)
        read_ascii 4 <<- EOF
				${c26} .==.${c7}##${c26}====.,
				${c26}/###|${c7}/\\${c26}|#######=
				${c7}__.__##__.____${c93}\`##\\
				${c7}""'""##""'""""#.${c93}\\#'
				${c26}####'${c7}\\/${c26}'#####/${c7}##${c93}/##
				${c26}\\###'${c7}##${c26}'####"${c7}##'${c93}###
				${c26} "##,${c7}\\#=..=###${c93}.##'
				${c26}  \`'#=,${c7}\`'"\`${c93}=###"
				${c93}     \`'+##='\`
			EOF
        ;;

    [Bb]edrock*)
        read_ascii 4 <<- EOF
				${c7}__
				${c7}\\ \\___
				${c7} \\  _ \\
				${c7}  \\___/
			EOF
        ;;

    [Bb]uildroot*)
        read_ascii 3 <<- EOF
				${c3}   ___
				${c3} / \`   \\
				${c3}|   :  :|
				${c3}-. _:__.-
				${c3}  \` ---- \`
			EOF
        ;;

    [Cc]el[Oo][Ss]*)
        read_ascii 5 <<- EOF
				${c5}      .////\\\\\//\\.
				${c5}     //_         \\\\
				${c5}    /_  ${c7}##############
				${c5}   //              *\\
				${c7}###############    ${c5}|#
				${c5}   \/              */
				${c5}    \*   ${c7}##############
				${c5}     */,        .//
				${c5}      '_///\\\\\//_'
			EOF
        ;;

    [Cc]ent[Oo][Ss]*)
        read_ascii 5 <<- EOF
				${c2} ____${c3}^${c5}____
				${c2} |\\  ${c3}|${c5}  /|
				${c2} | \\ ${c3}|${c5} / |
				${c5}<---- ${c4}---->
				${c4} | / ${c2}|${c3} \\ |
				${c4} |/__${c2}|${c3}__\\|
				${c2}     v
			EOF
        ;;

    [Cc]rystal*[Ll]inux)
        read_ascii 5 <<- EOF
				${c5}        -//.
				${c5}      -//.
				${c5}    -//. .
				${c5}  -//.  '//-
				${c5} /+:      :+/
				${c5}  .//'  .//.
				${c5}    . .//.
				${c5}    .//.
				${c5}  .//.
			EOF
        ;;

    [Dd]ahlia*)
        read_ascii 1 <<- EOF
				${c1}      _
				${c1}  ___/ \\___
				${c1} |   _-_   |
				${c1} | /     \ |
				${c1}/ |       | \\
				${c1}\\ |       | /
				${c1} | \ _ _ / |
				${c1} |___ - ___|
				${c1}     \\_/
			EOF
        ;;

    [Dd]ebian*)
        read_ascii 1 <<- EOF
				${c1}  _____
				${c1} /  __ \\
				${c1}|  /    |
				${c1}|  \\___-
				${c1}-_
				${c1}  --_
			EOF
        ;;

    [Dd]evuan*)
        read_ascii 6 <<- EOF
				${c4} ..:::.
				${c4}    ..-==-
				${c4}        .+#:
				${c4}         =@@
				${c4}      :+%@#:
				${c4}.:=+#@@%*:
				${c4}#@@@#=:
			EOF
        ;;

    [Dd]iet[Pp]i*)
        read_ascii 8 2 <<- EOF
				${c2}  __  __
				${c2} (_\\)(/_)
				${c8} (______)
				${c8}(_${c2}<>${c8}__${c2}<>${c8}_)
				${c8} (__${c2}''${c8}__)
				${c8}   (__)
			EOF
        ;;

    [Dd]ragon[Ff]ly*)
        read_ascii 1 <<- EOF
				    ,${c1}_${c7},
				 ('-_${c1}|${c7}_-')
				  >--${c1}|${c7}--<
				 (_-'${c1}|${c7}'-_)
				     ${c1}|
				     ${c1}|
				     ${c1}|
			EOF
        ;;

    [Ee]lementary*)
        read_ascii <<- EOF
				${c7}  _______
				${c7} / ____  \\
				${c7}/  |  /  /\\
				${c7}|__\\ /  / |
				${c7}\\   /__/  /
				 ${c7}\\_______/
			EOF
        ;;

    [Ee]ndeavour*)
        read_ascii 4 <<- EOF
				      ${c1}/${c4}\\
				    ${c1}/${c4}/  \\${c6}\\
				   ${c1}/${c4}/    \\ ${c6}\\
				 ${c1}/ ${c4}/     _) ${c6})
				${c1}/_${c4}/___-- ${c6}__-
				 ${c6}/____--
			EOF
        ;;

    [Ff]edora*)
        read_ascii 4 <<- EOF
				        ${c4},'''''.
				       ${c4}|   ,.  |
				       ${c4}|  |  '_'
				${c4}  ,....|  |..
				${c4}.'  ,_;|   ..'
				${c4}|  |   |  |
				${c4}|  ',_,'  |
				${c4} '.     ,'
				   ${c4}'''''
			EOF
        ;;

    [Ff]iwix*)
        read_ascii 12 <<- EOF
				 ${c6}_____  ${c4}_____
				 ${c6}\\    \\ ${c4}\\    \\
				  ${c6}\\    \\ ${c4}\\    \\
				 ${c6}/ \\    \\ ${c4}\\    \\
				${c6}(   \\    \\ ${c4}\\    \\
				${c6}(   /    / ${c4}/    /
				 ${c6}\\ /    / ${c4}/    /
				  ${c6}/    / ${c4}/    /
				 ${c6}/____/ ${c4}/____/
			EOF
        ;;

    [Ff]ree[Bb][Ss][Dd]*)
        read_ascii 1 <<- EOF
				${c1}/\\,-'''''-,/\\
				${c1}\\_)       (_/
				${c1}|           |
				${c1}|           |
				 ${c1};         ;
				  ${c1}'-_____-'
			EOF
        ;;

    [Gg]aruda*)
        read_ascii 4 <<- EOF
				${c3}         _______
				${c3}      __/       \\_
				${c3}    _/     /      \\_
				${c7}  _/      /_________\\
				${c7}_/                  |
				${c2}\\     ____________
				${c2} \\_            __/
				${c2}   \\__________/
			EOF
        ;;

    [Gg]entoo*)
        read_ascii 5 <<- EOF
				${c5} _-----_
				${c5}(       \\
				${c5}\\    0   \\
				${c7} \\        )
				${c7} /      _/
				${c7}(     _-
				${c7}\\____-
			EOF
        ;;

    ([Gg][Nn][Uu]*|[Hh]urd*)
        read_ascii 7 <<-EOF
				${c7} ---${c3}[ ]${c7}<----
				${c7} v   |      |
				${c3}[ ]${c7}  --->${c3}[ ]${c7}|
				${c7} ^        | |
				${c7} |  ${c3}[ ]${c7}---|--
				${c7}  ---------
			EOF
        ;;

    [Gg]uix[Ss][Dd]* | [Gg]uix*)
        read_ascii 3 <<- EOF
				${c3}|.__          __.|
				${c3}|__ \\        / __|
				   ${c3}\\ \\      / /
				    ${c3}\\ \\    / /
				     ${c3}\\ \\  / /
				      ${c3}\\ \\/ /
				       ${c3}\\__/
			EOF
        ;;

    [Hh]aiku*)
        read_ascii 3 <<- EOF
				${c3}       ,^,
				 ${c3}     /   \\
				${c3}*--_ ;     ; _--*
				${c3}\\   '"     "'   /
				 ${c3}'.           .'
				${c3}.-'"         "'-.
				 ${c3}'-.__.   .__.-'
				       ${c3}|_|
			EOF
        ;;

    [Hh]ydroOS*)
        read_ascii 4 <<- EOF
				${c1}╔╗╔╗──╔╗───╔═╦══╗
				${c1}║╚╝╠╦╦╝╠╦╦═╣║║══╣
				${c1}║╔╗║║║╬║╔╣╬║║╠══║
				${c1}╚╝╚╬╗╠═╩╝╚═╩═╩══╝
				${c1}───╚═╝
			EOF
        ;;

    [Hh]yperbola*)
        read_ascii <<- EOF
				${c7}    |\`__.\`/
				   ${c7} \____/
				   ${c7} .--.
				  ${c7} /    \\
				 ${c7} /  ___ \\
				 ${c7}/ .\`   \`.\\
				${c7}/.\`      \`.\\
			EOF
        ;;

    [Ii]glunix*)
        read_ascii <<- EOF
				${c7}       |
				${c7}       |          |
				${c7}                  |
				${c7}  |    ________
				${c7}  |  /\\   |    \\
				${c7}    /  \\  |     \\  |
				${c7}   /    \\        \\ |
				${c7}  /      \\________\\
				${c7}  \\      /        /
				${c7}   \\    /        /
				${c7}    \\  /        /
				${c7}     \\/________/
			EOF
        ;;

    [Ii]nstant[Oo][Ss]*)
        read_ascii <<- EOF
				${c7} ,-''-,
				${c7}: .''. :
				${c7}: ',,' :
				${c7} '-____:__
				${c7}       :  \`.
				${c7}       \`._.'
			EOF
        ;;

    [Ii][Rr][Ii][Xx]*)
        read_ascii 1 <<- EOF
				${c1} __
				${c1} \\ \\   __
				${c1}  \\ \\ / /
				${c1}   \\ v /
				${c1}   / . \\
				${c1}  /_/ \\ \\
				${c1}       \\_\\
			EOF
        ;;

    [Kk][Dd][Ee]*[Nn]eon*)
        read_ascii 6 <<- EOF
				${c7}   .${c6}__${c7}.${c6}__${c7}.
				${c6}  /  _${c7}.${c6}_  \\
				${c6} /  /   \\  \\
				${c7} . ${c6}|  ${c7}O${c6}  | ${c7}.
				${c6} \\  \\_${c7}.${c6}_/  /
				${c6}  \\${c7}.${c6}__${c7}.${c6}__${c7}.${c6}/
			EOF
        ;;

    [Ll]inux*[Ll]ite* | [Ll]ite*)
        read_ascii 3 <<- EOF
				${c3}   /\\
				${c3}  /  \\
				${c3} / ${c7}/ ${c3}/
			${c3}> ${c7}/ ${c3}/
				${c3}\\ ${c7}\\ ${c3}\\
				 ${c3}\\_${c7}\\${c3}_\\
				${c7}    \\
			EOF
        ;;

    [Ll]inux*[Mm]int* | [Mm]int)
        read_ascii 2 <<- EOF
				${c2} ___________
				${c2}|_          \\
				  ${c2}| ${c7}| _____ ${c2}|
				  ${c2}| ${c7}| | | | ${c2}|
				  ${c2}| ${c7}| | | | ${c2}|
				  ${c2}| ${c7}\\__${c7}___/ ${c2}|
				  ${c2}\\_________/
			EOF
        ;;

    [Ll]inux*)
        read_ascii 4 <<- EOF
				${c4}    ___
				   ${c4}(${c7}.. ${c4}|
				   ${c4}(${c5}<> ${c4}|
				  ${c4}/ ${c7}__  ${c4}\\
				 ${c4}( ${c7}/  \\ ${c4}/|
				${c5}_${c4}/\\ ${c7}__)${c4}/${c5}_${c4})
				${c5}\/${c4}-____${c5}\/
			EOF
        ;;

    [Mm]ac[Oo][Ss]* | [Dd]arwin*)
        read_ascii 1 <<- EOF
				${c2}       .:'
				${c2}    _ :'_
				${c3} .'\`_\`-'_\`\`.
				${c1}:________.-'
				${c1}:_______:
				${c4} :_______\`-;
				${c5}  \`._.-._.'
			EOF
        ;;

    [Mm]ageia*)
        read_ascii 2 <<- EOF
				${c6}   *
				${c6}    *
				${c6}   **
				${c7} /\\__/\\
				${c7}/      \\
				${c7}\\      /
				${c7} \\____/
			EOF
        ;;

    [Mm]anjaro*)
        read_ascii 2 <<- EOF
				${c2}||||||||| ||||
				${c2}||||||||| ||||
				${c2}||||      ||||
				${c2}|||| |||| ||||
				${c2}|||| |||| ||||
				${c2}|||| |||| ||||
				${c2}|||| |||| ||||
			EOF
        ;;

    [Mm]inix*)
        read_ascii 4 <<- EOF
				${c4} ,,        ,,
				${c4};${c7},${c4} ',    ,' ${c7},${c4};
				${c4}; ${c7}',${c4} ',,' ${c7},'${c4} ;
				${c4};   ${c7}',${c4}  ${c7},'${c4}   ;
				${c4};  ${c7};, '' ,;${c4}  ;
				${c4};  ${c7};${c4};${c7}',,'${c4};${c7};${c4}  ;
				${c4}', ${c7};${c4};;  ;;${c7};${c4} ,'
				 ${c4} '${c7};${c4}'    '${c7};${c4}'
			EOF
        ;;

    [Mm]orph[Oo][Ss]*)
        read_ascii 1 <<- EOF
				${c4}  __ \/ __
				${c4} /o \{}/ o\\
				${c4} \   ()   /
				${c4}  \`> /\ <\`
				${c4}  (o/\/\o)
				${c4}   )    (
			EOF
        ;;

    [Mm][Xx]*)
        read_ascii <<- EOF
				${c7}    \\\\  /
				 ${c7}    \\\\/
				 ${c7}     \\\\
				 ${c7}  /\\/ \\\\
				${c7}  /  \\  /\\
				${c7} /    \\/  \\
			${c7}/__________\\
			EOF
        ;;

    [Nn]et[Bb][Ss][Dd]*)
        read_ascii 3 <<- EOF
				${c7}\\\\${c3}\`-______,----__
				${c7} \\\\        ${c3}__,---\`_
				${c7}  \\\\       ${c3}\`.____
				${c7}   \\\\${c3}-______,----\`-
				${c7}    \\\\
				${c7}     \\\\
				${c7}      \\\\
			EOF
        ;;

    [Nn]ix[Oo][Ss]*)
        read_ascii 4 <<- EOF
				${c4}  \\\\  \\\\ //
				${c4} ==\\\\__\\\\/ //
				${c4}   //   \\\\//
				${c4}==//     //==
				${c4} //\\\\___//
				${c4}// /\\\\  \\\\==
				${c4}  // \\\\  \\\\
			EOF
        ;;

    [Nn]obara*)
        read_ascii 4 <<- EOF
				${c7} _._.  _..,._
				${c7}|##############.
				${c7}|################.
				${c7}|#####/  .  \\#####.
				${c7}|####|  ###   > ###
				${c7}|#####  \`\`\`  |#####
				${c7}|######==_   |#####
				${c7}|######"##|  |#####
				${c7} \`""\`\`    '  \`\\##/
			EOF
		;;

    [Oo]pen[Bb][Ss][Dd]*)
        read_ascii 3 <<- EOF
				${c3}      _____
				${c3}    \\-     -/
				${c3} \\_/         \\
				${c3} |        ${c7}O O${c3} |
				${c3} |_  <   )  3 )
				${c3} / \\         /
				 ${c3}   /-_____-\\
			EOF
        ;;

    [Oo]pen[Ss][Uu][Ss][Ee]*[Tt]umbleweed*)
        read_ascii 2 <<- EOF
				${c2}  _____   ______
				${c2} / ____\\ / ____ \\
				${c2}/ /    \`/ /    \\ \\
				${c2}\\ \\____/ /,____/ /
				${c2} \\______/ \\_____/
			EOF
        ;;

    [Oo]pen[Ss][Uu][Ss][Ee]* | [Oo]pen*SUSE* | SUSE* | suse*)
        read_ascii 2 <<- EOF
				${c2}  _______
				${c2}__|   __ \\
				${c2}     / .\\ \\
				${c2}     \\__/ |
				${c2}   _______|
				${c2}   \\_______
				${c2}__________/
			EOF
        ;;

    [Oo]pen[Ww]rt*)
        read_ascii 1 <<- EOF
				${c1} _______
				${c1}|       |.-----.-----.-----.
				${c1}|   -   ||  _  |  -__|     |
				${c1}|_______||   __|_____|__|__|
				${c1} ________|__|    __
				${c1}|  |  |  |.----.|  |_
				${c1}|  |  |  ||   _||   _|
				${c1}|________||__|  |____|
			EOF
        ;;

    [Oo]racle*)
        read_ascii 1 <<- EOF
				${c1}     ___________
				${c1} ./##+++++++++++##\\.
				${c1}/#/               \\#\\
				${c1}|#|               |#|
				${c1}\\#\\               /#/
				${c1} "##.............##"
				${c1}   \`'''''''''''''\`
			EOF
    	;;

    [Pp]arabola*)
        read_ascii 5 <<- EOF
				${c5}  __ __ __  _
				${c5}.\`_//_//_/ / \`.
				${c5}          /  .\`
				${c5}         / .\`
				${c5}        /.\`
				${c5}       /\`
			EOF
        ;;

    [Pp]op!_[Oo][Ss]*)
        read_ascii 6 <<- EOF
				${c6}     .///////,
				${c6}   //${c7}76767${c6}//////
				${c6}  //${c7}76${c6}//${c7}76${c6}//${c7}767${c6}//
				${c6} ////${c7}7676'${c6}//${c7}76${c6}////
				${c6} /////${c7}76${c6}////${c7}7${c6}/////
				${c6}  /////${c7}76${c6}//${c7}76${c6}////
				${c6}   ///${c7}76767676${c6}//
				${c6}     \`///////'
			EOF
        ;;

    [Pp]ure[Oo][Ss]*)
        read_ascii <<- EOF
				${c7} _____________
				${c7}|  _________  |
				${c7}| |         | |
				${c7}| |         | |
				${c7}| |_________| |
				${c7}|_____________|
			EOF
        ;;

    [Rr]aspbian*)
        read_ascii 1 <<- EOF
				${c2}  __  __
				${c2} (_\\)(/_)
				${c1} (_(__)_)
				${c1}(_(_)(_)_)
				${c1} (_(__)_)
				${c1}   (__)
			EOF
        ;;

    [Rr]ocky*)
        read_ascii 2 <<- EOF
				${c35}      __
				${c35}  .=######=.
				${c35}.###########'
				${c35}########${c7}.\\${c35}"###
				${c35}#####"${c7}/####=${c35}'#
				${c35}'##"${c7}###${c35}.##.${c7}##'
				${c35} '${c7}###${c35}.#####+\`
				${c35}    \`\`''\`\`
			EOF
		;;

    [Ss]erenity[Oo][Ss]*)
        read_ascii 4 <<- EOF
				${c7}    _____
				${c1}  ,-${c7}     -,
				${c1} ;${c7} (       ;
				${c1}| ${c7}. \_${c1}.,${c7}    |
				${c1}|  ${c7}o  _${c1} ',${c7}  |
				${c1} ;   ${c7}(_)${c1} )${c7} ;
				${c1}  '-_____-${c7}'
			EOF
        ;;

    [Ss]lackware*)
        read_ascii 4 <<- EOF
				${c4}   ________
				${c4}  /  ______|
				${c4}  | |______
				${c4}  \\______  \\
				${c4}   ______| |
				${c4}| |________/
				${c4}|____________
			EOF
        ;;

    [Ss]olus*)
        read_ascii 4 <<- EOF
				${c6}
				${c6}     /|
				${c6}    / |\\
				${c6}   /  | \\ _
				${c6}  /___|__\\_\\
				${c6} \\         /
				${c6}  \`-------´
			EOF
        ;;

    [Ss]team*)
        read_ascii 5 <<- EOF
				${c5}        ____
				${c5}    .=+#######=.
				${c5}  ;########-"""-#\\.
				${c5} ###${c7}####${c5}#/ .'\`'.'##.
				${c7}:########  |   | ${c5}|##
				${c7}\`''"=#='    \`'\`,.##${c5}#
				${c7}      \`'.  .+#######
				${c7}\`##.  .." #########
				${c7}  '=#=.,+########"
				${c7}    \`"=######="\`
			EOF
		;;

    [Ss]un[Oo][Ss] | [Ss]olaris*)
        read_ascii 3 <<- EOF
				${c3}       .   .;   .
				${c3}   .   :;  ::  ;:   .
				${c3}   .;. ..      .. .;.
				${c3}..  ..             ..  ..
				${c3} .;,                 ,;.
			EOF
        ;;

    [Uu]buntu*)
        read_ascii 3 <<- EOF
				${c3}         _
				${c3}     ---(_)
				${c3} _/  ---  \\
				${c3}(_) |   |
				 ${c3} \\  --- _/
				    ${c3} ---(_)
			EOF
        ;;

	[Vv]anilla*)
	    read_ascii 3 <<- EOF
				${c3}        .#.
				${c3}       :#=#:
				${c3}.#=":.:##=##:.:"=#.
				${c3}\`:#=#####=####=##:\`
				${c3} \`:####=\` \`=####:\`
				${c3}   .:##,. .,##:.
				${c3}  :##=##:-:##=##:
				${c3} .#=##:\`   \`:##=#.
				${c3}  \`\`           \`\`
			EOF
        ;;

    [Vv]oid*)
        read_ascii 2 <<- EOF
				${c2}    _______
				${c2} _ \\______ -
				${c2}| \\  ___  \\ |
				${c2}| | /   \ | |
				${c2}| | \___/ | |
				${c2}| \\______ \\_|
				${c2} -_______\\
			EOF
        ;;

    [Ww]indows*)
        read_ascii 4 <<- EOF
				${c4}######  ######
				${c4}######  ######
				${c4}######  ######
				${c4}
				${c4}######  ######
				${c4}######  ######
				${c4}######  ######
			EOF
        ;;
    [Xx]eonix*)
        read_ascii 2 <<- EOF
				${c2}    ___  ___
				${c2}___ \  \/  / ___
				${c2}\  \ \    / /  /
				${c2} \  \/    \/  /
				${c2}  \    /\    /
				${c2}   \__/  \__/
			EOF
        ;;
esac

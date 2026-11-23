"""Pages written for lore directly, for commands that neither tldr nor the
man pages cover well (revise.py adds them to the drafts):
name → (summary, [(what to type, what it does)], notes or None)."""

PAGES = {
    ":": (
        "A bash builtin that does nothing and always succeeds (exit status 0). It is useful where the shell needs a command but you want no action, and its arguments are still expanded.",
        [
            (":", "do nothing and succeed"),
            ("while :; do command; sleep 1; done", "loop forever, running a command once a second"),
            ("if condition; then :; else echo no; fi", "leave a branch empty without a syntax error"),
            (": \"${VAR:=default}\"", "set a variable to a default if it is unset, without running anything"),
            (": > file.txt", "create an empty file, or empty an existing one"),
        ],
        "`: > file.txt` erases the content of an existing file.",
    ),
    "setpriv": (
        "Runs a program with chosen Linux privileges, capabilities and security settings, without needing a setuid helper.",
        [
            ("setpriv --reuid=1000 --regid=1000 --clear-groups id", "run a program as another user and group, dropping extra groups"),
            ("setpriv --no-new-privs program", "run a program that can never gain privileges through setuid or file capabilities"),
            ("setpriv --bounding-set=-all program", "remove every capability from the bounding set for the program"),
            ("setpriv --dump", "show the current privilege state of the shell"),
        ],
        "Changing the user needs root. Check the result with `--dump` before relying on it.",
    ),
    "killall5": (
        "Sends a signal to every process outside its own session, the way the System V init scripts did at shutdown.",
        [
            ("killall5 -15", "ask every such process to terminate (SIGTERM)"),
            ("killall5 -9", "kill every such process without letting it clean up"),
            ("killall5 -9 -o 1234", "kill all of them except the process with that id"),
        ],
        "Run as root this can kill nearly everything on the machine, including your login. Do not try it on a desktop.",
    ),
    "mawk": (
        "A fast implementation of AWK, the small language for picking apart and summing columns of text.",
        [
            ("mawk '{ print $1 }' file.txt", "print the first column of every line"),
            ("mawk -F: '{ print $1 }' /etc/passwd", "use a colon as the field separator"),
            ("mawk '{ s += $2 } END { print s }' file.txt", "add up the second column"),
            ("mawk '/error/' file.log", "print only the lines that match a pattern"),
            ("mawk -f script.awk file.txt", "run a program stored in a file"),
        ],
        None,
    ),
    "nawk": (
        "The system's AWK program (here the mawk implementation): a small language for processing text columns.",
        [
            ("nawk '{ print $1 }' file.txt", "print the first column of every line"),
            ("nawk -F, '{ print $2 }' data.csv", "split lines on commas and print the second field"),
            ("nawk 'NR > 1' file.txt", "skip the first line"),
            ("nawk '{ n++ } END { print n }' file.txt", "count the lines"),
        ],
        None,
    ),
    "traceroute6": (
        "Shows the path IPv6 packets take to a host, one router (hop) per line, with the time each hop takes.",
        [
            ("traceroute6 example.com", "trace the route to a host over IPv6"),
            ("traceroute6 -n example.com", "do not resolve names, which is faster"),
            ("traceroute6 -m 20 example.com", "give up after 20 hops"),
            ("traceroute6 -I example.com", "probe with ICMP echo packets instead of UDP"),
        ],
        "Lines of stars mean a router did not answer. That is common and does not by itself mean a problem.",
    ),
    "xdg-email": (
        "Opens the user's default mail program with a new message already filled in.",
        [
            ("xdg-email someone@example.com", "start a new message to an address"),
            ("xdg-email --subject 'Hello' --body 'Text' someone@example.com", "set the subject and body"),
            ("xdg-email --attach report.pdf someone@example.com", "attach a file"),
            ("xdg-email --cc a@example.com --bcc b@example.com c@example.com", "add copy and blind-copy recipients"),
        ],
        None,
    ),
    "which.debianutils": (
        "Prints the full path of the program a command name would run, by searching the folders in PATH.",
        [
            ("which.debianutils ls", "print where ls is"),
            ("which.debianutils -a python3", "print every match in PATH, not only the first"),
            ("which.debianutils -s ls", "print nothing and only set the exit status"),
        ],
        None,
    ),
    "update-grub2": (
        "Regenerates the GRUB boot menu file from the installed kernels and the system's settings.",
        [
            ("update-grub2", "rebuild the boot menu"),
            ("update-grub2 -o /boot/grub/grub.cfg", "name the output file explicitly (normally the default)"),
        ],
        "Needs root. Edit `/etc/default/grub` first, then run this for the change to take effect. A broken boot menu can stop the machine from starting.",
    ),
    "dpkg-divert": (
        "Manages diversions: redirects where a package's file is installed so a local version is not overwritten on upgrade.",
        [
            ("dpkg-divert --list", "show every diversion in place"),
            ("dpkg-divert --add --rename /usr/bin/tool", "move a file aside and divert future installs of it"),
            ("dpkg-divert --remove --rename /usr/bin/tool", "undo a diversion and move the file back"),
            ("dpkg-divert --list '*tool*'", "show only diversions matching a pattern"),
        ],
        "Needs root. Remove a diversion before removing the package that owns the file, or the package system gets confused.",
    ),
    "tclsh": (
        "The interpreter for Tcl, a small scripting language. Runs a script, or opens an interactive prompt.",
        [
            ("tclsh", "start an interactive session"),
            ("tclsh script.tcl", "run a script"),
            ("tclsh script.tcl arg1 arg2", "run a script with arguments, read from argv"),
            ("echo 'puts [expr 2+2]' | tclsh", "run commands from a pipe"),
        ],
        None,
    ),
    "tclsh8.6": (
        "The Tcl 8.6 interpreter: runs Tcl scripts or an interactive prompt.",
        [
            ("tclsh8.6", "start an interactive session"),
            ("tclsh8.6 script.tcl", "run a script"),
            ("tclsh8.6 script.tcl arg1", "run a script with an argument"),
        ],
        None,
    ),
    "rsync-ssl": (
        "A wrapper that runs rsync over a TLS-encrypted connection to an rsync daemon.",
        [
            ("rsync-ssl -av host::module/ local/", "copy a module from a daemon into a folder, over TLS"),
            ("rsync-ssl --type=openssl -av host::module/ local/", "choose the TLS helper program explicitly"),
            ("rsync-ssl -av local/ host::module/", "send a folder to the daemon"),
        ],
        None,
    ),
    "nc.openbsd": (
        "The OpenBSD netcat: reads and writes data over TCP or UDP connections, for testing ports and moving bytes between machines.",
        [
            ("nc.openbsd -zv host 80", "check whether a port is open"),
            ("nc.openbsd -l 9000", "listen on a port and print what arrives"),
            ("nc.openbsd host 9000 < file", "send a file to a listening machine"),
            ("nc.openbsd -u host 53", "use UDP instead of TCP"),
            ("nc.openbsd -w 5 host 80", "give up after 5 seconds"),
        ],
        "Send only to machines you own or may test.",
    ),
    "firefox-esr": (
        "Mozilla Firefox, Extended Support Release: a graphical web browser.",
        [
            ("firefox-esr", "open the browser"),
            ("firefox-esr https://example.com", "open a page"),
            ("firefox-esr --private-window", "open a private window"),
            ("firefox-esr --new-window https://example.com", "open a page in a new window"),
            ("firefox-esr -P", "pick or manage profiles"),
        ],
        None,
    ),
    "brave-browser": (
        "The Brave web browser, a Chromium-based browser with a built-in ad blocker.",
        [
            ("brave-browser", "open the browser"),
            ("brave-browser https://example.com", "open a page"),
            ("brave-browser --incognito", "open a private window"),
            ("brave-browser --new-window https://example.com", "open a page in a new window"),
        ],
        None,
    ),
    "wireshark": (
        "A graphical network protocol analyzer: captures packets from an interface or opens a capture file and lets you inspect them.",
        [
            ("wireshark", "open the program"),
            ("wireshark capture.pcap", "open a saved capture file"),
            ("wireshark -i eth0 -k", "start capturing on an interface at once"),
            ("wireshark -Y 'http' capture.pcap", "open a file with a display filter applied"),
        ],
        "Capturing live traffic usually needs extra permission. Only capture on networks you are allowed to monitor.",
    ),
    "mergecap": (
        "Combines several packet capture files into one.",
        [
            ("mergecap -w out.pcap a.pcap b.pcap", "merge files in time order into out.pcap"),
            ("mergecap -a -w out.pcap a.pcap b.pcap", "append the files one after another instead of by time"),
            ("mergecap -F pcapng -w out.pcapng a.pcap b.pcap", "choose the output format"),
        ],
        None,
    ),
    "editcap": (
        "Edits or converts packet capture files: cut out packets, change the format, split a big capture.",
        [
            ("editcap -r in.pcap out.pcap 1-100", "keep only packets 1 to 100"),
            ("editcap -d in.pcap out.pcap", "remove duplicate packets"),
            ("editcap -F pcap in.pcapng out.pcap", "convert to another capture format"),
            ("editcap -c 10000 in.pcap part.pcap", "split into files of 10000 packets each"),
        ],
        None,
    ),
    "capinfos": (
        "Prints summary information about packet capture files: format, packet count, size, and time span.",
        [
            ("capinfos capture.pcap", "show a summary of a capture"),
            ("capinfos -c capture.pcap", "show only the packet count"),
            ("capinfos -T capture.pcap", "print the summary as a table"),
        ],
        None,
    ),
    "text2pcap": (
        "Turns a hex dump of packets into a capture file Wireshark can read.",
        [
            ("text2pcap dump.txt out.pcap", "convert a hex dump to a capture file"),
            ("text2pcap -e 0x800 dump.txt out.pcap", "add an Ethernet header with a given type"),
            ("text2pcap -u 1000,2000 dump.txt out.pcap", "wrap the data in a UDP header"),
        ],
        None,
    ),
    "socat1": (
        "A relay that connects two data channels, such as a file, a socket, a port or a terminal, and copies bytes between them.",
        [
            ("socat - TCP:host:80", "talk to a TCP port from your terminal"),
            ("socat TCP-LISTEN:8080,fork TCP:host:80", "forward a local port to another host"),
            ("socat -u FILE:in.txt TCP:host:9000", "send a file over TCP"),
            ("socat UNIX-LISTEN:/tmp/sock,fork TCP:host:80", "expose a TCP service through a unix socket"),
        ],
        "A forward like this opens a door in the network. Close it when you are done.",
    ),
    "rmiregistry": (
        "Starts a registry for Java remote objects (RMI) on a port.",
        [
            ("rmiregistry", "start the registry on the default port 1099"),
            ("rmiregistry 2000", "start the registry on another port"),
        ],
        "RMI is rarely used in new code, and an open registry is a risk on an untrusted network.",
    ),
    "jshell": (
        "An interactive Java prompt: type Java statements and see the result at once.",
        [
            ("jshell", "start an interactive session"),
            ("jshell script.jsh", "run a script file and stay in the prompt"),
            ("jshell --class-path lib.jar", "start with a library on the class path"),
            ("jshell -s", "start in concise mode, with less output"),
        ],
        None,
    ),
    "jcmd": (
        "Sends diagnostic commands to a running Java process.",
        [
            ("jcmd", "list running Java processes"),
            ("jcmd 1234 VM.version", "show the JVM version of a process"),
            ("jcmd 1234 Thread.print", "print the thread stacks of a process"),
            ("jcmd 1234 GC.heap_info", "show the heap usage"),
            ("jcmd 1234 help", "list the commands that process accepts"),
        ],
        None,
    ),
    "jstat": (
        "Prints statistics from a running Java virtual machine, such as garbage collection and memory use.",
        [
            ("jstat -gc 1234", "show garbage collector figures once"),
            ("jstat -gcutil 1234 1000", "show memory use in percent every second"),
            ("jstat -class 1234", "show class loading figures"),
        ],
        None,
    ),
    "jconsole": (
        "A graphical tool for watching and managing a running Java process: memory, threads and classes.",
        [
            ("jconsole", "pick a local process to connect to"),
            ("jconsole 1234", "connect to a process by id"),
            ("jconsole host:9999", "connect to a remote JVM that exposes JMX"),
        ],
        None,
    ),
    "jlink": (
        "Builds a small custom Java runtime that contains only the modules an application needs.",
        [
            ("jlink --add-modules java.base --output rt", "create a runtime with the base module only"),
            ("jlink --add-modules java.base,java.sql --output rt", "include more modules"),
            ("jlink --list-plugins", "show the optimisation plugins available"),
        ],
        None,
    ),
    "jmod": (
        "Creates and inspects JMOD files, the packaging format for Java modules.",
        [
            ("jmod list file.jmod", "list the content of a module file"),
            ("jmod describe file.jmod", "show a module's description"),
            ("jmod create --class-path classes out.jmod", "create a module file from classes"),
        ],
        None,
    ),
    "jpackage": (
        "Packages a Java application, with its runtime, into a native installer or app image.",
        [
            ("jpackage --name App --input lib --main-jar app.jar --main-class Main", "build an installer for the current system"),
            ("jpackage --type app-image --name App --input lib --main-jar app.jar", "build a plain app folder, no installer"),
            ("jpackage --type deb --name App --input lib --main-jar app.jar", "build a Debian package"),
        ],
        None,
    ),
    "jfr": (
        "Prints and summarises Java Flight Recorder files, which hold performance recordings.",
        [
            ("jfr summary recording.jfr", "show what the recording contains"),
            ("jfr print --events GC recording.jfr", "print the garbage collection events"),
            ("jfr print recording.jfr", "print every event"),
        ],
        None,
    ),
    "jdb": (
        "The Java debugger: set breakpoints and step through a Java program from the terminal.",
        [
            ("jdb MainClass", "start a class under the debugger"),
            ("jdb -attach 5005", "attach to a JVM started with a debug port"),
            ("jdb -sourcepath src MainClass", "tell the debugger where the sources are"),
        ],
        None,
    ),
    "jinfo": (
        "Shows or changes the settings and system properties of a running Java process.",
        [
            ("jinfo 1234", "show every property and flag of a process"),
            ("jinfo -flags 1234", "show only the JVM flags"),
            ("jinfo -sysprops 1234", "show only the system properties"),
        ],
        None,
    ),
    "jrunscript": (
        "Runs scripts written for the Java scripting API. It needs a script engine installed.",
        [
            ("jrunscript", "start an interactive prompt"),
            ("jrunscript -f script.js", "run a script file"),
            ("jrunscript -e 'print(1+2)'", "run one expression"),
        ],
        None,
    ),
    "jwebserver": (
        "Starts a minimal web server that serves the files of a folder, for testing.",
        [
            ("jwebserver", "serve the current folder on port 8000"),
            ("jwebserver -p 9000", "use another port"),
            ("jwebserver -d /path/to/folder", "serve another folder"),
            ("jwebserver -b 0.0.0.0", "accept connections from other machines"),
        ],
        "By default it listens only on this machine. Binding to 0.0.0.0 shares the folder with the network.",
    ),
    "nodejs": (
        "The Node.js runtime: runs JavaScript outside the browser.",
        [
            ("nodejs", "start an interactive session"),
            ("nodejs script.js", "run a script"),
            ("nodejs -e 'console.log(1+1)'", "run one expression"),
            ("nodejs --version", "print the version"),
            ("nodejs --watch script.js", "re-run the script when it changes"),
        ],
        None,
    ),
    "python3.13": (
        "The Python 3.13 interpreter: runs Python programs or an interactive prompt.",
        [
            ("python3.13", "start an interactive session"),
            ("python3.13 script.py", "run a script"),
            ("python3.13 -m venv .venv", "create a virtual environment"),
            ("python3.13 -m http.server 8000", "serve the current folder on port 8000"),
            ("python3.13 -c 'print(1+1)'", "run one line of code"),
        ],
        None,
    ),
    "php8.4": (
        "The PHP 8.4 command-line interpreter.",
        [
            ("php8.4 script.php", "run a script"),
            ("php8.4 -r 'echo 1+1;'", "run one line of code"),
            ("php8.4 -S localhost:8000", "start the built-in web server in the current folder"),
            ("php8.4 -l script.php", "check a file's syntax"),
            ("php8.4 -m", "list the loaded modules"),
        ],
        None,
    ),
    "ruby3.3": (
        "The Ruby 3.3 interpreter.",
        [
            ("ruby3.3 script.rb", "run a script"),
            ("ruby3.3 -e 'puts 1+1'", "run one line of code"),
            ("ruby3.3 -c script.rb", "check a file's syntax"),
            ("ruby3.3 -v", "print the version"),
        ],
        None,
    ),
    "irb3.3": (
        "The interactive Ruby prompt (IRB): type Ruby code and see each result.",
        [
            ("irb3.3", "start a session"),
            ("irb3.3 -r json", "start with a library loaded"),
            ("irb3.3 --simple-prompt", "use a shorter prompt"),
        ],
        None,
    ),
    "ghc-9.6.6": (
        "The Glasgow Haskell Compiler, version 9.6.6.",
        [
            ("ghc-9.6.6 Main.hs", "compile a program"),
            ("ghc-9.6.6 -O2 Main.hs -o app", "compile with optimisation to a named binary"),
            ("ghc-9.6.6 -e 'print 1'", "evaluate one expression"),
            ("ghc-9.6.6 --version", "print the compiler version"),
        ],
        None,
    ),
    "ghci-9.6.6": (
        "The interactive Haskell prompt of GHC 9.6.6.",
        [
            ("ghci-9.6.6", "start a session"),
            ("ghci-9.6.6 Main.hs", "load a file into the session"),
            ("ghci-9.6.6 -e 'print 1'", "evaluate one expression"),
        ],
        None,
    ),
    "runghc": (
        "Runs a Haskell program straight from its source, without compiling it first.",
        [
            ("runghc Main.hs", "run a program"),
            ("runghc Main.hs arg1", "run it with an argument"),
            ("runghc -e 'print 1'", "evaluate one expression"),
        ],
        None,
    ),
    "luarocks": (
        "The package manager for Lua modules (rocks).",
        [
            ("luarocks install name", "install a module"),
            ("luarocks list", "show installed modules"),
            ("luarocks search name", "search for a module"),
            ("luarocks remove name", "uninstall a module"),
            ("luarocks install --local name", "install into your home folder instead of the system"),
        ],
        "Installing without `--local` writes to system folders and needs root.",
    ),
    "lua5.4": (
        "The Lua 5.4 interpreter: runs Lua scripts or an interactive prompt.",
        [
            ("lua5.4", "start an interactive session"),
            ("lua5.4 script.lua", "run a script"),
            ("lua5.4 -e 'print(1+1)'", "run one line of code"),
            ("lua5.4 -v", "print the version"),
        ],
        None,
    ),
    "lua5.1": (
        "The Lua 5.1 interpreter: runs Lua scripts or an interactive prompt.",
        [
            ("lua5.1", "start an interactive session"),
            ("lua5.1 script.lua", "run a script"),
            ("lua5.1 -e 'print(1+1)'", "run one line of code"),
        ],
        None,
    ),
    "perl5.40.1": (
        "The Perl 5.40.1 interpreter.",
        [
            ("perl5.40.1 script.pl", "run a script"),
            ("perl5.40.1 -e 'print 1+1'", "run one line of code"),
            ("perl5.40.1 -ne 'print if /text/' file", "print the lines of a file that match"),
            ("perl5.40.1 -pi -e 's/old/new/g' file", "replace text in a file in place"),
        ],
        "`-pi` edits the file with no backup. Add a suffix, as in `-pi.bak`, to keep one.",
    ),
    "prove": (
        "Runs Perl test files and prints a short summary of which passed.",
        [
            ("prove t/", "run every test in the t folder"),
            ("prove -v t/basic.t", "run one test file with full output"),
            ("prove -r t/", "search folders recursively for tests"),
            ("prove -j4 t/", "run four tests at a time"),
        ],
        None,
    ),
    "pod2man": (
        "Converts Perl POD documentation into a manual page.",
        [
            ("pod2man script.pl > script.1", "write a man page from a script's POD"),
            ("pod2man --section=1 script.pl script.1", "set the manual section"),
            ("pod2man --name=tool script.pl > tool.1", "set the page name"),
        ],
        None,
    ),
    "pod2text": (
        "Converts Perl POD documentation to plain text.",
        [
            ("pod2text script.pl", "print the documentation of a script"),
            ("pod2text script.pl out.txt", "write it to a file"),
        ],
        None,
    ),
    "pod2html": (
        "Converts Perl POD documentation to an HTML page.",
        [
            ("pod2html --infile=script.pl --outfile=doc.html", "write an HTML page from a script's POD"),
            ("pod2html --title='My tool' --infile=script.pl --outfile=doc.html", "set the page title"),
        ],
        None,
    ),
    "perldoc": (
        "Shows the documentation of Perl modules, functions and guides.",
        [
            ("perldoc perlintro", "read the Perl introduction"),
            ("perldoc -f print", "look up a built-in function"),
            ("perldoc Module::Name", "read a module's documentation"),
            ("perldoc -q word", "search the FAQ"),
        ],
        None,
    ),
    "piconv": (
        "Converts text from one character encoding to another, a Perl take on iconv.",
        [
            ("piconv -f latin1 -t utf-8 in.txt", "convert a file from Latin-1 to UTF-8"),
            ("piconv -l", "list the known encodings"),
        ],
        None,
    ),
    "json_pp": (
        "Pretty-prints JSON, reading from a file or standard input.",
        [
            ("json_pp < data.json", "pretty-print a JSON file"),
            ("echo '{\"a\":1}' | json_pp", "pretty-print JSON from a pipe"),
            ("json_pp -json_opt canonical < data.json", "sort the keys for a stable output"),
        ],
        None,
    ),
    "xclip-copyfile": (
        "Copies files to the clipboard so they can be pasted elsewhere, using xclip.",
        [
            ("xclip-copyfile file.txt", "copy one file to the clipboard"),
            ("xclip-copyfile a.txt b.txt", "copy several files"),
        ],
        None,
    ),
    "xclip-pastefile": (
        "Pastes the files held in the clipboard (put there by xclip-copyfile) into the current folder.",
        [
            ("xclip-pastefile", "paste the copied files here"),
        ],
        None,
    ),
    "xclip-cutfile": (
        "Cuts files to the clipboard, so that pasting them also removes the originals.",
        [
            ("xclip-cutfile file.txt", "cut a file to the clipboard"),
            ("xclip-pastefile", "paste, which moves the file here"),
        ],
        "The originals are removed when you paste.",
    ),
    "xdg-screensaver": (
        "Controls the screensaver from a script: activate, lock, reset or suspend it.",
        [
            ("xdg-screensaver lock", "lock the screen"),
            ("xdg-screensaver activate", "start the screensaver"),
            ("xdg-screensaver reset", "reset the idle timer"),
            ("xdg-screensaver status", "show whether the screensaver is enabled"),
        ],
        None,
    ),
    "xdg-icon-resource": (
        "Installs icons for the desktop so menus and launchers can use them.",
        [
            ("xdg-icon-resource install --size 48 icon.png myapp", "install a 48 pixel icon under a name"),
            ("xdg-icon-resource uninstall --size 48 myapp", "remove an installed icon"),
        ],
        None,
    ),
    "xdg-desktop-icon": (
        "Installs or removes a launcher file on the user's desktop folder.",
        [
            ("xdg-desktop-icon install myapp.desktop", "put a launcher on the desktop"),
            ("xdg-desktop-icon uninstall myapp.desktop", "remove it again"),
        ],
        None,
    ),
    "xsetroot": (
        "Sets the look of the X desktop background: colour, pattern or cursor.",
        [
            ("xsetroot -solid grey", "paint the background a solid colour"),
            ("xsetroot -cursor_name left_ptr", "use the normal arrow cursor"),
            ("xsetroot -name text", "set the root window name, often read by a status bar"),
        ],
        None,
    ),
    "xdpyinfo": (
        "Prints information about the X display: screens, extensions and visuals.",
        [
            ("xdpyinfo", "show everything about the display"),
            ("xdpyinfo | grep dimensions", "show the screen size"),
            ("xdpyinfo -ext all", "list every extension"),
        ],
        None,
    ),
    "xmessage": (
        "Shows a small window with a message and buttons, handy in shell scripts.",
        [
            ("xmessage 'Done'", "show a message"),
            ("xmessage -buttons yes,no 'Continue?'", "show a question and read the answer from the exit code"),
            ("xmessage -timeout 5 'Done'", "close the window after 5 seconds"),
        ],
        None,
    ),
    "xkbcomp": (
        "Compiles keyboard layout descriptions, and can dump or load the current X keymap.",
        [
            ("xkbcomp :0 layout.xkb", "save the current keymap to a file"),
            ("xkbcomp layout.xkb :0", "load a keymap from a file"),
        ],
        "A bad keymap can make the keyboard unusable until you restart the X session.",
    ),
    "xfconf-query": (
        "Reads and changes settings in the Xfce settings database from the terminal.",
        [
            ("xfconf-query -l", "list the channels"),
            ("xfconf-query -c xsettings -l", "list the properties in a channel"),
            ("xfconf-query -c xsettings -p /Net/ThemeName", "read a value"),
            ("xfconf-query -c xsettings -p /Net/ThemeName -s Adwaita", "set a value"),
        ],
        None,
    ),
    "glxinfo": (
        "Prints information about the OpenGL driver and the graphics card in use.",
        [
            ("glxinfo | grep 'OpenGL renderer'", "show which GPU renders"),
            ("glxinfo -B", "show a short summary"),
            ("glxinfo | grep 'direct rendering'", "check whether hardware rendering works"),
        ],
        None,
    ),
    "setpci": (
        "Reads and writes the configuration registers of PCI devices.",
        [
            ("setpci -s 00:1f.2 COMMAND", "read one register of a device"),
            ("setpci -L", "list the register names it knows"),
        ],
        "Writing the wrong value to a device can freeze the system. Needs root.",
    ),
    "usb-devices": (
        "Lists the USB devices attached, with details such as vendor, product and driver.",
        [
            ("usb-devices", "show every USB device and its details"),
        ],
        None,
    ),
    "usbreset": (
        "Resets a USB device, as if it was unplugged and plugged in again.",
        [
            ("usbreset", "list the devices with their bus and ids"),
            ("usbreset 1234:abcd", "reset a device by vendor and product id"),
            ("usbreset 001/004", "reset a device by bus and device number"),
        ],
        "Needs root. Whatever uses the device loses its connection for a moment.",
    ),
    "sensors-detect": (
        "Probes the machine for hardware monitoring chips and says which kernel modules to load for `sensors`.",
        [
            ("sensors-detect", "start the interactive scan"),
            ("sensors-detect --auto", "accept the default answers"),
        ],
        "Needs root. The scan touches hardware registers, so read each question before you answer.",
    ),
    "update-pciids": (
        "Downloads the newest list of PCI vendor and device names, used by `lspci`.",
        [
            ("update-pciids", "fetch and install the new list"),
        ],
        "Needs root and an internet connection.",
    ),
}

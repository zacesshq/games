-- Learn to Code: the Bash and Linux track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- Runs of spaces count as one but a space between words matters, as in a real shell.
-- The examples run as the user ada in /home/ada.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[Bash and Linux]==],
  id = [==[bash]==],
  quotes_equal = true,
  keep_spaces = true,
  levels = {
    {
      name = [==[Saying something]==], topic = [==[Output]==],
      lesson = { [==[echo prints its arguments.]==] },
      code = { [==[{1} "Hello, world"]==] },
      blanks = {
        { answers = { [==[echo]==] }, hint = [==[The command that prints text.]==], wrong = { { [==[print]==], [==[Shells print with echo (or printf).]==] }, { [==[printf]==], [==[printf works too but needs \n at the end. echo is the simple one.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Where am I?]==], topic = [==[Navigation]==],
      lesson = { [==[pwd prints the working directory.]==] },
      code = { [==[{1}]==] },
      blanks = {
        { answers = { [==[pwd]==] }, hint = [==[Print working directory.]==], wrong = { { [==[cd]==], [==[cd moves. pwd shows where you are.]==] }, { [==[ls]==], [==[ls lists what is here. pwd shows where here is.]==] } } },
      },
      output = { [==[/home/ada]==] },
    },
    {
      name = [==[Moving around]==], topic = [==[Navigation]==],
      lesson = { [==[cd changes directory. cd .. goes up one level and cd on its own goes home.]==] },
      code = { [==[{1} projects]==] },
      blanks = {
        { answers = { [==[cd]==] }, hint = [==[Change directory.]==], wrong = { { [==[pwd]==], [==[pwd only shows where you are. cd moves.]==] } } },
      },
      output = { [==[now in /home/ada/projects]==] },
    },
    {
      name = [==[A long listing]==], topic = [==[Files]==],
      lesson = { [==[ls lists files. -l shows the long form with permissions, sizes and dates.]==] },
      code = { [==[ls {1}]==] },
      blanks = {
        { answers = { [==[-l]==], [==[-la]==], [==[-al]==], [==[-lh]==], [==[-lah]==] }, hint = [==[The option for the long listing.]==], wrong = { { [==[-a]==], [==[-a adds hidden files but keeps the short form. -l is the long one.]==] }, { [==[l]==], [==[Options start with a dash: -l.]==] } } },
      },
      output = { [==[-rw-r--r-- 1 ada ada 120 Oct 29 notes.txt]==] },
    },
    {
      name = [==[Folders in folders]==], topic = [==[Files]==],
      lesson = { [==[mkdir makes a directory. -p makes any missing parents too.]==] },
      code = { [==[mkdir {1} a/b/c]==] },
      blanks = {
        { answers = { [==[-p]==] }, hint = [==[The option for parents.]==], wrong = { { [==[-r]==], [==[mkdir has no -r. -p makes the parents.]==] } } },
      },
      output = { [==[a, a/b and a/b/c now exist]==] },
    },
    {
      name = [==[Copying a folder]==], topic = [==[Files]==],
      lesson = { [==[cp copies files. -r copies a directory and everything in it.]==] },
      code = { [==[cp {1} src backup]==] },
      blanks = {
        { answers = { [==[-r]==], [==[-R]==], [==[-a]==] }, hint = [==[The option for recursive.]==], wrong = { { [==[-p]==], [==[-p keeps times and modes but does not go into folders. -r does.]==] } } },
      },
      output = { [==[backup is a full copy of src]==] },
    },
    {
      name = [==[Renaming]==], topic = [==[Files]==],
      lesson = { [==[mv moves or renames.]==] },
      code = { [==[{1} draft.txt final.txt]==] },
      blanks = {
        { answers = { [==[mv]==] }, hint = [==[Two letters, short for move.]==], wrong = { { [==[rename]==], [==[rename exists but works differently. mv renames one file.]==] }, { [==[cp]==], [==[cp would leave draft.txt behind.]==] } } },
      },
      output = { [==[draft.txt is now final.txt]==] },
    },
    {
      name = [==[Making a script runnable]==], topic = [==[Permissions]==],
      lesson = { [==[chmod changes permissions. +x adds permission to run.]==] },
      code = { [==[chmod {1} run.sh]==] },
      blanks = {
        { answers = { [==[+x]==], [==[u+x]==], [==[755]==], [==[744]==], [==[a+x]==] }, hint = [==[Add the x permission.]==], wrong = { { [==[x]==], [==[Say add with a plus: +x.]==] }, { [==[777]==], [==[777 lets everyone change the file too. +x or 755 is enough.]==] } } },
      },
      output = { [==[./run.sh now runs]==] },
    },
    {
      name = [==[Passing output along]==], topic = [==[Pipes]==],
      lesson = { [==[| sends one command's output into the next command's input.]==] },
      code = { [==[cat log.txt {1} wc -l]==] },
      blanks = {
        { answers = { [==[|]==] }, hint = [==[The vertical bar.]==], wrong = { { [==[>]==], [==[> writes into a file named wc. | passes the output to wc.]==] } } },
      },
      output = { [==[the number of lines in log.txt]==] },
    },
    {
      name = [==[Into a file]==], topic = [==[Redirection]==],
      lesson = { [==[> writes output to a file, replacing what was there. >> adds to the end.]==] },
      code = { [==[ls {1} files.txt]==] },
      blanks = {
        { answers = { [==[>]==] }, hint = [==[One greater than sign.]==], wrong = { { [==[|]==], [==[| passes output to a program. > writes it to a file.]==] }, { [==[<]==], [==[< reads a file into a command. > writes output out.]==] } } },
      },
      output = { [==[files.txt holds the listing]==] },
    },
    {
      name = [==[Adding to a log]==], topic = [==[Redirection]==],
      lesson = { [==[>> appends output to the end of a file.]==] },
      code = { [==[echo "done" {1} log.txt]==] },
      blanks = {
        { answers = { [==[>>]==] }, hint = [==[Two greater than signs.]==], wrong = { { [==[>]==], [==[> would replace the whole log with just done.]==] } } },
      },
      output = { [==[done is added as the last line]==] },
    },
    {
      name = [==[Finding lines]==], topic = [==[grep]==],
      lesson = { [==[grep prints lines that match a pattern. -i ignores case.]==] },
      code = { [==[{1} -i error log.txt]==] },
      blanks = {
        { answers = { [==[grep]==] }, hint = [==[The search command.]==], wrong = { { [==[find]==], [==[find looks for files. grep looks inside them.]==] }, { [==[search]==], [==[The command is grep.]==] } } },
      },
      output = { [==[every line mentioning error in any case]==] },
    },
    {
      name = [==[Swapping text]==], topic = [==[sed]==],
      lesson = { [==[sed 's/old/new/' replaces the first match on each line. Add g after the last slash for every match.]==] },
      code = { [==[sed 's/{1}/V/' readings.txt]==] },
      blanks = {
        { answers = { [==[volts]==] }, hint = [==[The word being replaced, as it appears in the file: volts.]==], wrong = { { [==[V]==], [==[The new text is already there after the second slash.]==] } } },
      },
      output = { [==[5 volts becomes 5 V]==] },
    },
    {
      name = [==[Picking columns]==], topic = [==[awk]==],
      lesson = { [==[awk splits each line into fields $1, $2 and so on, by spaces unless told otherwise.]==] },
      code = { [==[awk '{print ${1}}' parts.txt]==] },
      blanks = {
        { answers = { [==[2]==] }, hint = [==[The second field.]==], wrong = { { [==[1]==], [==[$1 is the first field, the name. The voltage is in $2.]==] } } },
      },
      output = { [==[the second column of every line]==] },
    },
    {
      name = [==[Finding files]==], topic = [==[find]==],
      lesson = { [==[find searches under a folder. -name matches a file name pattern; quote it so the shell leaves the * alone.]==] },
      code = { [==[find . -name {1}]==] },
      blanks = {
        { answers = { [==["*.py"]==], [==[*.py]==] }, hint = [==[Every name ending in .py.]==], wrong = { { [==[.py]==], [==[-name matches the whole name, so it needs a * before .py.]==] } } },
      },
      output = { [==[every Python file under here]==] },
    },
    {
      name = [==[What is running?]==], topic = [==[Processes]==],
      lesson = { [==[ps lists processes. ps aux shows everyone's.]==] },
      code = { [==[{1} aux | grep python]==] },
      blanks = {
        { answers = { [==[ps]==] }, hint = [==[Two letters for process status.]==], wrong = { { [==[top]==], [==[top shows a live view and does not pipe well. ps prints a list.]==] } } },
      },
      output = { [==[the python processes running]==] },
    },
    {
      name = [==[Stopping a program]==], topic = [==[Processes]==],
      lesson = { [==[kill sends a signal to a process by its number. The default asks it to stop.]==] },
      code = { [==[{1} 4321]==] },
      blanks = {
        { answers = { [==[kill]==] }, hint = [==[The command that signals a process.]==], wrong = { { [==[stop]==], [==[The command is kill.]==] }, { [==[rm]==], [==[rm deletes files. kill stops processes.]==] } } },
      },
      output = { [==[process 4321 is asked to stop]==] },
    },
    {
      name = [==[Your settings]==], topic = [==[Environment]==],
      lesson = { [==[Environment variables hold settings. $NAME reads one.]==] },
      code = { [==[echo ${1}]==] },
      blanks = {
        { answers = { [==[HOME]==] }, hint = [==[The variable holding your home directory.]==], wrong = { { [==[home]==], [==[Variable names are case sensitive: HOME.]==] } } },
      },
      output = { [==[/home/ada]==] },
    },
    {
      name = [==[Passing settings on]==], topic = [==[Environment]==],
      lesson = { [==[export makes a variable visible to the programs you start.]==] },
      code = { [==[{1} EDITOR=nvim]==] },
      blanks = {
        { answers = { [==[export]==] }, hint = [==[The word that shares a variable with child programs.]==], wrong = { { [==[set]==], [==[set changes shell options. export shares a variable.]==] }, { [==[let]==], [==[let does arithmetic. export shares a variable.]==] } } },
      },
      output = { [==[programs started now see EDITOR=nvim]==] },
    },
    {
      name = [==[The first line of a script]==], topic = [==[Scripting]==],
      lesson = { [==[A script starts with #! and the program that runs it.]==] },
      code = { [==[#!{1}]==], [==[echo "Ready"]==] },
      blanks = {
        { answers = { [==[/bin/bash]==], [==[/usr/bin/env bash]==] }, hint = [==[The path to bash.]==], wrong = { { [==[bash]==], [==[#! needs a full path, such as /bin/bash.]==] } } },
      },
      output = { [==[Ready]==] },
    },
    {
      name = [==[Script arguments]==], topic = [==[Scripting]==],
      lesson = { [==[Inside a script $1 is the first argument, $2 the second and so on.]==], [==[Here the script runs as ./greet.sh Ada.]==] },
      code = { [==[echo "Hello ${1}"]==] },
      blanks = {
        { answers = { [==[1]==] }, hint = [==[The number of the first argument.]==], wrong = { { [==[0]==], [==[$0 is the script's own name.]==] }, { [==[@]==], [==[$@ is every argument. The first one is $1.]==] } } },
      },
      output = { [==[Hello Ada]==] },
    },
    {
      name = [==[Checking first]==], topic = [==[Scripting]==],
      lesson = { [==[if [ test ]; then ... fi runs a block when the test is true. -f tests for a file.]==] },
      code = { [==[if [ -f config.txt ]; {1}]==], [==[  echo "Found"]==], [==[fi]==] },
      blanks = {
        { answers = { [==[then]==] }, hint = [==[The word after the test.]==], wrong = { { [==[do]==], [==[do starts a loop body. if uses then.]==] } } },
      },
      output = { [==[Found]==] },
    },
    {
      name = [==[Every text file]==], topic = [==[Scripting]==],
      lesson = { [==[for name in list; do ... done runs a block for each item.]==] },
      code = { [==[for f in *.txt; {1} wc -l "$f"; done]==] },
      blanks = {
        { answers = { [==[do]==] }, hint = [==[The word that starts the loop's body.]==], wrong = { { [==[then]==], [==[then belongs to if. A loop body starts with do.]==] } } },
      },
      output = { [==[the line count of each .txt file]==] },
    },
    {
      name = [==[Every morning]==], topic = [==[cron]==],
      lesson = { [==[A crontab line gives minute, hour, day, month and weekday, then the command. crontab -e edits it.]==] },
      code = { [==[{1} 9 * * * ./backup.sh]==] },
      blanks = {
        { answers = { [==[0]==] }, hint = [==[The minute past the hour.]==], wrong = { { [==[9]==], [==[The first field is the minute. 9 here would mean 9 minutes past every hour.]==] }, { [==[*]==], [==[* in the first field would run it every minute.]==] } } },
      },
      output = { [==[backup.sh runs at 09:00 every day]==] },
    },
    {
      name = [==[Another computer]==], topic = [==[ssh]==],
      lesson = { [==[ssh logs in to another computer: ssh user@host.]==] },
      code = { [==[{1} ada@server.example]==] },
      blanks = {
        { answers = { [==[ssh]==] }, hint = [==[Secure shell.]==], wrong = { { [==[telnet]==], [==[telnet is unencrypted. Use ssh.]==] }, { [==[scp]==], [==[scp copies files. ssh logs in.]==] } } },
      },
      output = { [==[a shell on server.example]==] },
    },
    {
      name = [==[Installing tools]==], topic = [==[Package managers]==],
      lesson = { [==[Ubuntu and Debian install with apt, Fedora with dnf and macOS with brew.]==] },
      code = { [==[sudo {1} install git]==] },
      blanks = {
        { answers = { [==[apt]==], [==[apt-get]==], [==[dnf]==], [==[yum]==] }, hint = [==[The package manager of a Linux distribution.]==], wrong = { { [==[brew]==], [==[brew is for macOS and does not need sudo.]==] }, { [==[pip]==], [==[pip installs Python packages, not system tools.]==] } } },
      },
      output = { [==[git is installed]==] },
    },
  },
}

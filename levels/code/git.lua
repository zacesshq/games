-- Learn to Code: the Git track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- Runs of spaces count as one but a space between words matters.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[Git]==],
  id = [==[git]==],
  quotes_equal = true,
  keep_spaces = true,
  levels = {
    {
      name = [==[A new repository]==], topic = [==[Basics]==],
      lesson = { [==[git init turns the current folder into a repository.]==] },
      code = { [==[git {1}]==] },
      blanks = {
        { answers = { [==[init]==] }, hint = [==[Short for initialise.]==], wrong = { { [==[start]==], [==[The command is git init.]==] }, { [==[new]==], [==[The command is git init.]==] } } },
      },
      output = { [==[Initialized empty Git repository]==] },
    },
    {
      name = [==[What has changed?]==], topic = [==[Basics]==],
      lesson = { [==[git status shows changed, staged and untracked files.]==] },
      code = { [==[git {1}]==] },
      blanks = {
        { answers = { [==[status]==] }, hint = [==[The state of the working tree.]==], wrong = { { [==[log]==], [==[log shows commits. status shows what has changed since the last one.]==] } } },
      },
      output = { [==[On branch main]==] },
    },
    {
      name = [==[Choosing what to commit]==], topic = [==[Commits]==],
      lesson = { [==[git add stages changes for the next commit.]==] },
      code = { [==[git {1} notes.txt]==] },
      blanks = {
        { answers = { [==[add]==], [==[stage]==] }, hint = [==[The command that stages.]==], wrong = { { [==[commit]==], [==[commit records what is staged. First stage with add.]==] } } },
      },
      output = { [==[notes.txt is staged]==] },
    },
    {
      name = [==[Recording it]==], topic = [==[Commits]==],
      lesson = { [==[git commit records the staged changes. -m gives the message.]==] },
      code = { [==[git commit {1} "Add notes"]==] },
      blanks = {
        { answers = { [==[-m]==] }, hint = [==[The option for a message.]==], wrong = { { [==[-a]==], [==[-a stages tracked files but still opens an editor for the message. -m gives the message.]==] } } },
      },
      output = { [==[[main a1b2c3d] Add notes]==] },
    },
    {
      name = [==[The story so far]==], topic = [==[History]==],
      lesson = { [==[git log lists commits. --oneline shows one line each.]==] },
      code = { [==[git log {1}]==] },
      blanks = {
        { answers = { [==[--oneline]==] }, hint = [==[One line each.]==], wrong = { { [==[-1]==], [==[-1 shows only the last commit.]==] }, { [==[--short]==], [==[The option is --oneline.]==] } } },
      },
      output = { [==[a1b2c3d Add notes]==] },
    },
    {
      name = [==[What exactly changed]==], topic = [==[History]==],
      lesson = { [==[git diff shows changes not yet staged.]==] },
      code = { [==[git {1}]==] },
      blanks = {
        { answers = { [==[diff]==] }, hint = [==[The command that shows differences.]==], wrong = { { [==[status]==], [==[status lists files. diff shows the lines.]==] } } },
      },
      output = { [==[+ a new line]==] },
    },
    {
      name = [==[A branch of your own]==], topic = [==[Branches]==],
      lesson = { [==[git switch -c makes a branch and moves to it.]==] },
      code = { [==[git switch {1} feature]==] },
      blanks = {
        { answers = { [==[-c]==] }, hint = [==[The option to create.]==], wrong = { { [==[-b]==], [==[-b is for git checkout -b. git switch uses -c.]==] } } },
      },
      output = { [==[Switched to a new branch 'feature']==] },
    },
    {
      name = [==[Bringing work together]==], topic = [==[Merge]==],
      lesson = { [==[git merge brings another branch's commits into the current branch.]==] },
      code = { [==[git switch main]==], [==[git {1} feature]==] },
      blanks = {
        { answers = { [==[merge]==] }, hint = [==[The command that joins branches.]==], wrong = { { [==[rebase]==], [==[rebase replays your commits on another branch. merge joins feature into main.]==] }, { [==[pull]==], [==[pull fetches from a remote first. A local branch is joined with merge.]==] } } },
      },
      output = { [==[feature's commits are now on main]==] },
    },
    {
      name = [==[A straight history]==], topic = [==[Rebase]==],
      lesson = { [==[git rebase replays your branch's commits on top of another, for a straight line of history.]==] },
      code = { [==[git switch feature]==], [==[git {1} main]==] },
      blanks = {
        { answers = { [==[rebase]==] }, hint = [==[The command that replays commits.]==], wrong = { { [==[merge]==], [==[merge would add a merge commit. rebase replays the commits on top of main.]==] } } },
      },
      output = { [==[feature now starts from the tip of main]==] },
    },
    {
      name = [==[Where it lives online]==], topic = [==[Remotes]==],
      lesson = { [==[git remote add names a remote repository.]==] },
      code = { [==[git remote {1} origin git@github.com:ada/notes.git]==] },
      blanks = {
        { answers = { [==[add]==] }, hint = [==[The word that adds a remote.]==], wrong = { { [==[set-url]==], [==[set-url changes a remote that already exists. add makes a new one.]==] } } },
      },
      output = { [==[origin points at GitHub]==] },
    },
    {
      name = [==[Sending it up]==], topic = [==[Remotes]==],
      lesson = { [==[git push sends commits. -u sets the upstream so later pushes need no names.]==] },
      code = { [==[git push {1} origin feature]==] },
      blanks = {
        { answers = { [==[-u]==], [==[--set-upstream]==] }, hint = [==[The option that remembers the upstream.]==], wrong = { { [==[-f]==], [==[-f forces and can overwrite others' work. -u sets the upstream.]==] } } },
      },
      output = { [==[feature is on GitHub]==] },
    },
    {
      name = [==[Getting the latest]==], topic = [==[Remotes]==],
      lesson = { [==[git pull fetches and merges the remote branch.]==] },
      code = { [==[git {1}]==] },
      blanks = {
        { answers = { [==[pull]==] }, hint = [==[Fetch and merge in one.]==], wrong = { { [==[fetch]==], [==[fetch downloads but does not merge. pull does both.]==] }, { [==[push]==], [==[push sends your commits. pull brings others' in.]==] } } },
      },
      output = { [==[Already up to date.]==] },
    },
    {
      name = [==[A copy to work on]==], topic = [==[Remotes]==],
      lesson = { [==[git clone copies a repository and its history.]==] },
      code = { [==[git {1} https://github.com/ada/notes.git]==] },
      blanks = {
        { answers = { [==[clone]==] }, hint = [==[The command that copies a repository.]==], wrong = { { [==[copy]==], [==[The command is git clone.]==] }, { [==[pull]==], [==[pull updates a repository you already have.]==] } } },
      },
      output = { [==[Cloning into 'notes'...]==] },
    },
    {
      name = [==[Put it aside]==], topic = [==[Stash]==],
      lesson = { [==[git stash puts uncommitted changes aside so you can switch branches. git stash pop brings them back.]==] },
      code = { [==[git {1}]==] },
      blanks = {
        { answers = { [==[stash]==] }, hint = [==[Keep it safe for later.]==], wrong = { { [==[save]==], [==[git stash save is old. git stash is enough.]==] } } },
      },
      output = { [==[Saved working directory]==] },
    },
    {
      name = [==[Undo a file's changes]==], topic = [==[Undo]==],
      lesson = { [==[git restore throws away uncommitted changes to a file.]==] },
      code = { [==[git {1} notes.txt]==] },
      blanks = {
        { answers = { [==[restore]==] }, hint = [==[The command that puts a file back.]==], wrong = { { [==[reset]==], [==[reset moves branches and the index. restore puts a file back.]==] }, { [==[revert]==], [==[revert makes a new commit that undoes an old one.]==] } } },
      },
      output = { [==[notes.txt is back as last committed]==] },
    },
    {
      name = [==[Undo a commit safely]==], topic = [==[Undo]==],
      lesson = { [==[git revert makes a new commit that undoes an old one, without rewriting history.]==] },
      code = { [==[git {1} a1b2c3d]==] },
      blanks = {
        { answers = { [==[revert]==] }, hint = [==[The safe undo for shared history.]==], wrong = { { [==[reset --hard]==], [==[reset --hard rewrites history and loses work. revert is safe on a shared branch.]==] } } },
      },
      output = { [==[a new commit undoes a1b2c3d]==] },
    },
    {
      name = [==[Asking for review]==], topic = [==[Pull requests]==],
      lesson = { [==[A pull request asks for a branch to be reviewed and merged. With GitHub's gh tool: gh pr create.]==] },
      code = { [==[gh pr {1}]==] },
      blanks = {
        { answers = { [==[create]==] }, hint = [==[The word that opens a new one.]==], wrong = { { [==[open]==], [==[gh opens a pull request with gh pr create.]==] }, { [==[new]==], [==[gh opens a pull request with gh pr create.]==] } } },
      },
      output = { [==[https://github.com/ada/notes/pull/1]==] },
    },
  },
}

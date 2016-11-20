# Logrs

## What does it do?
It's a logger. Or rather it's there for you to use,
instead of for the things you write. You give it words, it logs those words
and keeps them well-fed and safe. Just like that:

```
$ logrs test!
Noted.
```

Logrs will also give you recaps of days:

```
$ logrs show today
 07:51 - Test for file display
 14:09 - test!
```

It sorts them neatly into daily files in ISO (yyyy-mm-dd) format, into a folder of your choosing.
(For which you currently have to edit the code, dotfile soon to come.)

### That's all? What about searching?
There's a `search` subcommand that wraps `grep`:
```
$ logrs search logrs
/Users/az/logs//2016-10-18: 10:59 - wheee more improvements to logrs
/Users/az/logs//2016-11-20: 16:13 - logrs test search string stuff.
```

### What's with the weird format?
The format is as it is to complement `grep`, so every entry is on one line, and grep doesn't just
give you fragments. Plus, it forms a mini timestamp with the file name, which is also nice.

## But Az, why does this exist?
Did you ever have a really obnoxious boss, that will rub things in your face
for months, should you forget or mess something up? I did. It wasn't great.
The latter part can't really be helped, but the 'forgetting things' part, is well
and easily solved by just entering a note into Logrs whenever you are done with
something, a commit, a function, a ticket, a meeting, a conversation, and so on.
This creates a very compact paper-ish trail that can be used to ward off annoying
and micromanaging bosses, like a voodoo charm.

## Okay, how do I get this thing?
- `git clone https://github.com/az4reus/logrs`
- `cd logrs`
- `cargo build --release`
- `ln -s /path/here/logrs/target/release/logrs /some/other/path/logrs`

## Aaaaz, what's to come in the future?
- [x] `logrs search` -- A simple wrapper for grep.
- [x] `logrs show 2015-01-01` -- Arbitrary date retrieval
- [ ] `logrs archive` -- Moving all things older to a month to a different folder, to avoid cluttering grep results. This will probably be automatic with a dotfile option.

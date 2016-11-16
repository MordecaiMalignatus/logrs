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

## That's all? What about searching?
Yes, that is all. For searching, there's `grep`. There's always `grep`.
To presume I could write a better, or even adequate grep is not likely, so instead,
the output has been formatted in a way that facilitates how `grep` works,
one line per entry. It also ends up being a not-quite-formally-correct timestamp.

```
$ grep -ri test ~/logs/
/Users/az/logs//2016-10-16: 07:51 - Test for file display
/Users/az/logs//2016-10-16: 14:09 - test!
```

A future patch may include an easier wrapper in the form of `logrs search`.

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
- [x]`logrs search` -- A simple wrapper for grep.
- [ ] `logrs archive` -- Moving all things older to a month to a different folder, to avoid cluttering grep results. This will probably be automatic with a dotfile option.
- [ ] `logrs show 2015-01-01` -- Arbitrary date retrieval, that includes archives.

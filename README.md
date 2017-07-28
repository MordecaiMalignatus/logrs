# Logrs

## What does it do?
It's a logger. Or rather it's there for you to use,
instead of for the things you write. You give it words, it logs those words
and keeps them well-fed and safe. Just like that:

```
$ logrs add test!
Noted. 01BP4JEXE2K174BDYAK00NTV84

$ logrs show 01BP4JEXE2K174BDYAK00NTV84
2017-07-28 14:24:10.946 +02:00 (01BP4JEXE2K174BDYAK00NTV84)
test!
```

Logrs will also give you recaps of days:

```
$ logrs show today
2017-07-28 11:20:23.671 +02:00 (01BP47YCKQ53DWVJTWENFWA0DA)
test 1

2017-07-28 14:24:10.946 +02:00 (01BP4JEXE2K174BDYAK00NTV84)
test!
```

You can specify any date and it will fetch all records for that date:

```
$ logrs show 2017-06-05
No records for 2017-06-05.

$ logrs show 2017-07-28
2017-07-28 11:20:23.671 +02:00 (01BP47YCKQ53DWVJTWENFWA0DA)
test 1

2017-07-28 14:24:10.946 +02:00 (01BP4JEXE2K174BDYAK00NTV84)
test!
```

You can also record multiple lines, or entire files from `stdin`:
```
$ logrs add -
You can write first line,
second line. Or any number of lines.
<Ctrl-d>
Noted. 01BP4Q35QDGF4MFBGH6BJQFZVY
```
Please note, the file needs to contain valid UTF-8 text.
```
$ logrs add - < somefile
Noted. 01BP4Q35QDGF4MFBGH6BJQFZVV

or

$ cat somefile | logrs add -
Noted. 01BP4Q35QDGF4MFBGH6BJQFZVV
```

It sorts them neatly into daily folders in ISO (yyyy-mm-dd) format, into a folder of your choosing.
(For which you currently have to edit the code, dotfile soon to come.)

### That's all? What about searching?
There's a `search` subcommand that wraps `grep`:
```
$ logrs search test
2017-07-27 14:00:45.672 +02:00 (01BP1YQA38CEVG5729RXY95BFQ):$ logrs test!
2017-07-27 14:00:45.672 +02:00 (01BP1YQA38CEVG5729RXY95BFQ): 07
2017-07-27 14:00:45.672 +02:00 (01BP1YQA38CEVG5729RXY95BFQ): 14
2017-07-27 14:00:45.672 +02:00 (01BP1YQA38CEVG5729RXY95BFQ):/Users/az/logs//2016-11-20
2017-07-28 14:24:10.946 +02:00 (01BP4JEXE2K174BDYAK00NTV84):test!
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
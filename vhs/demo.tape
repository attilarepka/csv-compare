Output vhs/demo.gif

Require csv-compare 

Set Shell zsh
Set Theme "tokyonight-storm"
Set Width 1800
Set Height 1000
Set Padding 20

Set TypingSpeed 100ms
Type "csv-compare vhs/orig.csv vhs/diff.csv --orig-index 1"
Enter
Sleep 2s
Type "y"
Enter
Sleep 5s
Hide
Type@1ms "rm orig.csv diff.csv"
Enter

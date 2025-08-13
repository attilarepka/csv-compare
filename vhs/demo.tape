Output vhs/demo.gif

Require csv-compare 

Set Shell zsh
Set Theme "tokyonight-storm"
Set Width 1800
Set Height 1000
Set Padding 20

Set TypingSpeed 1ms
Type 'echo -e "name,score,level\n\Alice,92,5\nBob,88,4\nCharlie,95,6\nDave,81,3\nEve,99,5\nFrank,74,2\nGrace,85,4\nHeidi,91,5\nIvan,77,3\nJudy,89,4\n\ Karl,83,3\nLaura,96,6\nMallory,72,2\nNiaj,80,3\nOlivia,94,5\nPeggy,78,3\nQuentin,82,4\nRupert,87,4\nSybil,90,5\nTrudy,75,2\n\ Uma,88,4\nVictor,90,5\nWendy,84,3\nXavier,93,6\nYvonne,91,4\nZack,86,3\nAnn,85,4\nBen,92,5\nCathy,87,3\nDan,89,4\n\ Ella,83,3\nFinn,95,6\nGina,78,2\nHank,80,3\nIvy,94,5\nJack,79,3\nKara,82,4\nLeo,88,4\nMia,91,5\nNora,76,2\n\ Owen,84,3\nPaul,96,6\nQuinn,73,2\nRalph,81,3\nSara,95,5\nTom,78,3\nUrsula,83,4\nVince,87,4\nWill,90,5\nZoe,75,2" > orig.csv'
Enter
Type 'echo -e "name,score,level\n\Alice,92,5\nBob,88,4\nCharlie,95,6\nDave,81,3\nEve,99,5\nFrank,74,2\nGrace,85,4\nHeidi,91,5\nIvan,77,3\nJudy,90,4\n\ Karl,83,3\nLaura,96,6\nMallory,72,2\nNiaj,80,3\nOlivia,94,5\nPeggy,78,3\nQuentin,82,4\nRupert,87,4\nSybil,90,5\nTrudy,76,2\n\ Uma,88,4\nVictor,90,5\nWendy,84,3\nXavier,93,6\nYvonne,91,4\nZack,86,3\nAnn,85,4\nBen,92,5\nCathy,87,3\nDan,90,4\n\ Ella,83,3\nFinn,95,6\nGina,78,2\nHank,80,3\nIvy,94,5\nJack,79,3\nKara,82,4\nLeo,88,4\nMia,91,5\nNora,77,2\n\ Owen,84,3\nPaul,96,6\nQuinn,73,2\nRalph,81,3\nSara,95,5\nTom,78,3\nUrsula,83,4\nVince,87,4\nWill,90,5\nZoe,76,2" > diff.csv'
Enter
Sleep 2s
Type "clear"
Enter
Sleep 250ms
Set TypingSpeed 100ms
Type "csv-compare orig.csv diff.csv --orig-index 2"
Enter
Sleep 2s
Type "y"
Enter
Sleep 5s
Hide
Type@1ms "rm orig.csv diff.csv"
Enter

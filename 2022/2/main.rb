

collected =  ARGF.each_line
    .map { |l| l.split(" ").map(&:ord) }
    .map { |o,s| [o-?A.ord, s-?X.ord] }

puts collected
    .map { |o,s| [s+3, s+6, s][(s-o+3)%3]+1 }
    .sum

puts collected
    .map { |o,s| [(o+2)%3, o+3, (o+1)%3+6 ][s]+1 }
    .sum

__END__
A Y
B X
C Z

0 1 [4, 7, 1][1]+1 8
1 0 [3, 6, 0][2]+1 1
2 2 [5, 8, 2][0]+1 6

0 1 [2, 3, 7][1]+1 4
1 0 [0, 4, 8][0]+1 1
2 2 [1, 5, 6][2]+1 7

entries = ARGF.each_line
  .map(&:chomp)
  .map { |l|
    l.split(",")
      .map { |pair| pair.split("-") }
      .map { |(l,r)| (l.to_i)..(r.to_i) }
  }

p entries.select { |(a,b)|
    (a.first <= b.first and a.last >= b.last) or (a.first >= b.first and a.last <= b.last)
}.size

p entries.reject { |(a,b)| a.last < b.first or a.first > b.last }.size

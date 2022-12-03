require 'set'
prio = [*(?a..?z),*(?A..?Z)]

lines = ARGF.each_line
    .map(&:chomp)
    .map(&:chars)

puts lines
    .map { |l| Set.new(l[0...l.size/2]) & Set.new(l[l.size/2..-1]) }
    .map { |s| prio.index(s.to_a.first)+1 }
    .sum

puts lines
    .map { |l| Set.new(l) }
    .each_slice(3)
    .map { |g| g.reduce(&:&) }
    .map { |b| prio.index(b.to_a.first)+1 }
    .sum

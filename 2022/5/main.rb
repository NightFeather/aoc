crates = []

loop do
  line = ARGF.readline.chomp
  break if line.empty?
  crates << line.scan(/(?:\[([A-Z])\]| ([0-9 ]) )(?: |$)/).flatten!.compact!
end
crates.pop
crates = crates.transpose.map { |s| i = s.find_index { |v| v != ' ' }; s[i..-1].reverse }

q1 = crates.map(&:dup)
q2 = crates.map(&:dup)

ARGF.each_line do |l|
  c, f, t = /move (\d+) from (\d+) to (\d+)/.match(l.chomp)[1..-1].map(&:to_i)
  tmp = q1[f-1].pop(c).reverse
  q1[t-1].push(*tmp)
  tmp = q2[f-1].pop(c)
  q2[t-1].push(*tmp)
end

puts q1.map(&:last).join
puts q2.map(&:last).join
